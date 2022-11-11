use self::{
    config::{IMTABLE_COLOMNS, VAR_COLUMNS},
    etable_compact::{EventTableChip, EventTableConfig},
    jtable::{JumpTableChip, JumpTableConfig},
    mtable_compact::{MemoryTableChip, MemoryTableConfig},
};
use crate::{
    circuits::{
        config::K,
        imtable::{InitMemoryTableConfig, MInitTableChip},
        itable::{InstructionTableChip, InstructionTableConfig},
        rtable::{RangeTableChip, RangeTableConfig},
        utils::Context,
    },
    foreign::{
        sha256_helper::{
            circuits::{assign::Sha256HelperTableChip, Sha256HelperTableConfig},
            SHA256_FOREIGN_TABLE_KEY,
        },
        wasm_input_helper::circuits::{
            assign::WasmInputHelperTableChip, WasmInputHelperTableConfig,
            WASM_INPUT_FOREIGN_TABLE_KEY,
        },
        ForeignTableConfig,
    },
};
use ark_std::{end_timer, start_timer};
use halo2_proofs::{
    arithmetic::{CurveAffine, FieldExt, MultiMillerLoop},
    circuit::{Layouter, SimpleFloorPlanner},
    pairing::bn256::{Bn256, Fr, G1Affine},
    plonk::{
        create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ConstraintSystem, Error,
        Expression, ProvingKey, SingleVerifier, VerifyingKey, VirtualCells,
    },
    poly::commitment::{Params, ParamsVerifier},
    transcript::{Blake2bRead, Blake2bWrite, EncodedChallenge, TranscriptRead, TranscriptWrite},
};
use num_bigint::BigUint;
use rand::rngs::OsRng;
use specs::{
    host_function::HostPlugin,
    itable::{OpcodeClass, OpcodeClassPlain},
    CompileTable, ExecutionTable,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Cursor, Read, Write},
    marker::PhantomData,
    path::PathBuf,
};

pub mod config;
pub mod etable_compact;
pub mod imtable;
pub mod itable;
pub mod jtable;
pub mod mtable_compact;
pub mod rtable;
pub mod utils;

pub(crate) trait FromBn {
    fn zero() -> Self;
    fn from_bn(bn: &BigUint) -> Self;
}

#[derive(Clone)]
pub struct TestCircuitConfig<F: FieldExt> {
    rtable: RangeTableConfig<F>,
    itable: InstructionTableConfig<F>,
    imtable: InitMemoryTableConfig<F>,
    mtable: MemoryTableConfig<F>,
    jtable: JumpTableConfig<F>,
    etable: EventTableConfig<F>,
    wasm_input_helper_table: WasmInputHelperTableConfig<F>,
    sha256_helper_table: Sha256HelperTableConfig<F>,
}

#[derive(Default, Clone)]
pub struct TestCircuit<F: FieldExt> {
    pub compile_tables: CompileTable,
    pub execution_tables: ExecutionTable,
    _data: PhantomData<F>,
}

impl<F: FieldExt> TestCircuit<F> {
    pub fn new(compile_tables: CompileTable, execution_tables: ExecutionTable) -> Self {
        TestCircuit {
            compile_tables,
            execution_tables,
            _data: PhantomData,
        }
    }
}

impl<F: FieldExt> Circuit<F> for TestCircuit<F> {
    type Config = TestCircuitConfig<F>;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let opcode_set = BTreeSet::from([
            OpcodeClassPlain(OpcodeClass::Br as usize),
            OpcodeClassPlain(OpcodeClass::BrIfEqz as usize),
            OpcodeClassPlain(OpcodeClass::Return as usize),
            OpcodeClassPlain(OpcodeClass::Drop as usize),
            OpcodeClassPlain(OpcodeClass::Call as usize),
            OpcodeClassPlain(OpcodeClass::Const as usize),
            OpcodeClassPlain(OpcodeClass::LocalGet as usize),
            OpcodeClassPlain(OpcodeClass::LocalSet as usize),
            OpcodeClassPlain(OpcodeClass::LocalTee as usize),
            OpcodeClassPlain(OpcodeClass::GlobalGet as usize),
            OpcodeClassPlain(OpcodeClass::GlobalSet as usize),
            OpcodeClassPlain(OpcodeClass::Bin as usize),
            OpcodeClassPlain(OpcodeClass::BinBit as usize),
            OpcodeClassPlain(OpcodeClass::BinShift as usize),
            OpcodeClassPlain(OpcodeClass::BrIf as usize),
            OpcodeClassPlain(OpcodeClass::Load as usize),
            OpcodeClassPlain(OpcodeClass::Store as usize),
            OpcodeClassPlain(OpcodeClass::Rel as usize),
            OpcodeClassPlain(OpcodeClass::Select as usize),
            OpcodeClassPlain(OpcodeClass::Test as usize),
            OpcodeClassPlain(OpcodeClass::Conversion as usize),
            OpcodeClassPlain(
                OpcodeClass::ForeignPluginStart as usize + HostPlugin::HostInput as usize,
            ),
            OpcodeClassPlain(
                OpcodeClass::ForeignPluginStart as usize + HostPlugin::Sha256 as usize,
            ),
        ]);

        let constants = meta.fixed_column();
        meta.enable_constant(constants);
        meta.enable_equality(constants);

        let mut cols = [(); VAR_COLUMNS].map(|_| meta.advice_column()).into_iter();

        let rtable = RangeTableConfig::configure([0; 7].map(|_| meta.lookup_table_column()));
        let itable = InstructionTableConfig::configure(meta.lookup_table_column());
        let imtable = InitMemoryTableConfig::configure(
            [0; IMTABLE_COLOMNS].map(|_| meta.lookup_table_column()),
        );
        let mtable = MemoryTableConfig::configure(meta, &mut cols, &rtable, &imtable);
        let jtable = JumpTableConfig::configure(meta, &mut cols, &rtable);

        let wasm_input_helper_table = WasmInputHelperTableConfig::configure(meta, &rtable);
        let sha256_helper_table = Sha256HelperTableConfig::configure(meta, &rtable);

        let mut foreign_tables = BTreeMap::<&'static str, Box<dyn ForeignTableConfig<_>>>::new();
        foreign_tables.insert(
            WASM_INPUT_FOREIGN_TABLE_KEY,
            Box::new(wasm_input_helper_table.clone()),
        );
        foreign_tables.insert(
            SHA256_FOREIGN_TABLE_KEY,
            Box::new(sha256_helper_table.clone()),
        );

        let etable = EventTableConfig::configure(
            meta,
            &mut cols,
            &rtable,
            &itable,
            &mtable,
            &jtable,
            &foreign_tables,
            &opcode_set,
        );

        Self::Config {
            rtable,
            itable,
            imtable,
            mtable,
            jtable,
            etable,
            wasm_input_helper_table,
            sha256_helper_table,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let rchip = RangeTableChip::new(config.rtable);
        let ichip = InstructionTableChip::new(config.itable);
        let imchip = MInitTableChip::new(config.imtable);
        let mchip = MemoryTableChip::new(config.mtable);
        let jchip = JumpTableChip::new(config.jtable);
        let echip = EventTableChip::new(config.etable);
        let wasm_input_chip = WasmInputHelperTableChip::new(config.wasm_input_helper_table);
        let sha256chip = Sha256HelperTableChip::new(config.sha256_helper_table);

        rchip.init(&mut layouter)?;
        wasm_input_chip.init(&mut layouter)?;
        sha256chip.init(&mut layouter)?;

        sha256chip.assign(
            &mut layouter,
            &self
                .execution_tables
                .etable
                .filter_foreign_entries(HostPlugin::Sha256),
        )?;
        wasm_input_chip.assign(
            &mut layouter,
            &self
                .execution_tables
                .etable
                .filter_foreign_entries(HostPlugin::HostInput),
        )?;

        ichip.assign(&mut layouter, &self.compile_tables.itable)?;
        if self.compile_tables.imtable.0.len() > 0 {
            imchip.assign(&mut layouter, &self.compile_tables.imtable)?;
        }

        layouter.assign_region(
            || "jtable mtable etable",
            |region| {
                let mut ctx = Context::new(region);

                let (rest_mops_cell, rest_jops_cell) =
                    { echip.assign(&mut ctx, &self.execution_tables.etable)? };

                ctx.reset();
                mchip.assign(&mut ctx, &self.execution_tables.mtable, rest_mops_cell)?;

                ctx.reset();
                jchip.assign(&mut ctx, &self.execution_tables.jtable, rest_jops_cell)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

trait Encode {
    fn encode(&self) -> BigUint;
}

pub(self) trait Lookup<F: FieldExt> {
    fn encode(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F>;

    fn configure_in_table(
        &self,
        meta: &mut ConstraintSystem<F>,
        key: &'static str,
        expr: impl FnOnce(&mut VirtualCells<'_, F>) -> Expression<F>,
    ) {
        meta.lookup_any(key, |meta| vec![(expr(meta), self.encode(meta))]);
    }
}

pub struct ZkWasmCircuitBuilder<
    C: CurveAffine,
    E: MultiMillerLoop<G1Affine = C>,
    Encode: EncodedChallenge<C>,
    W: TranscriptWrite<C, Encode>,
    R: TranscriptRead<C, Encode>,
> {
    pub circuit: TestCircuit<C::ScalarExt>,
    _mark_c: PhantomData<C>,
    _mark_e: PhantomData<E>,
    _mark_encode: PhantomData<Encode>,
    _mark_w: PhantomData<W>,
    _mark_r: PhantomData<R>,
}

const PARAMS: &str = "param.data";
const VK: &str = "vk.data";
const PROOF: &str = "vk.data";

impl<
        C: CurveAffine,
        E: MultiMillerLoop<G1Affine = C, Scalar = C::ScalarExt>,
        Encode: EncodedChallenge<C>,
        W: TranscriptWrite<C, Encode>,
        R: TranscriptRead<C, Encode>,
    > ZkWasmCircuitBuilder<C, E, Encode, W, R>
{
    pub fn new(compile_tables: CompileTable, execution_tables: ExecutionTable) -> Self {
        Self {
            circuit: TestCircuit::new(compile_tables, execution_tables),
            _mark_c: PhantomData,
            _mark_e: PhantomData,
            _mark_encode: PhantomData,
            _mark_w: PhantomData,
            _mark_r: PhantomData,
        }
    }

    fn create_params(&self) -> Params<C> {
        // Initialize the polynomial commitment parameters
        let timer = start_timer!(|| format!("build params with K = {}", K));
        let params: Params<C> = Params::<C>::unsafe_setup::<E>(K);
        end_timer!(timer);

        params
    }

    pub fn prepare_param(&self, path: Option<PathBuf>) -> Params<E::G1Affine> {
        let name = format!("{}.{}", K, PARAMS);

        let path = path.map(|path| path.as_path().join(name));

        if let Some(path) = &path {
            if path.exists() {
                let mut fd = File::open(path.as_path()).unwrap();
                let mut buf = vec![];

                fd.read_to_end(&mut buf).unwrap();
                return Params::<C>::read(Cursor::new(buf)).unwrap();
            }
        }

        let params = self.create_params();

        if let Some(path) = &path {
            let mut fd = File::create(path).unwrap();
            params.write(&mut fd).unwrap();
        }

        params
    }

    pub fn prepare_vk(
        &self,
        params: &Params<E::G1Affine>,
        cache: Option<PathBuf>,
    ) -> VerifyingKey<C> {
        let path = cache.map(|path| path.join(VK));

        if let Some(path) = &path {
            let mut fd = fs::File::open(path).unwrap();

            return VerifyingKey::read::<_, TestCircuit<C::ScalarExt>>(&mut fd, params).unwrap();
        }

        let timer = start_timer!(|| "build vk");
        let vk = keygen_vk(params, &self.circuit).expect("keygen_vk should not fail");
        end_timer!(timer);

        if let Some(path) = &path {
            let mut fd = fs::File::create(path).unwrap();
            vk.write(&mut fd).unwrap();
        }

        vk
    }

    pub fn prepare_pk(&self, params: &Params<C>, vk: VerifyingKey<C>) -> ProvingKey<C> {
        let timer = start_timer!(|| "build pk");
        let pk = keygen_pk(&params, vk, &self.circuit).expect("keygen_pk should not fail");
        end_timer!(timer);
        pk
    }

    /// For debug purpose
    pub fn try_load_proof(&self, path: Option<PathBuf>) -> Option<Vec<u8>> {
        let path = path.map(|path| path.join(PROOF));

        if path.as_ref().map_or(false, |p| p.exists()) {
            let mut buf = vec![];

            let mut fd = fs::File::open(path.unwrap()).unwrap();
            fd.read_to_end(&mut buf).unwrap();
            Some(buf)
        } else {
            None
        }
    }

    /// For debug purpose
    pub fn try_save_proof(&self, proof: &Vec<u8>, path: Option<PathBuf>) {
        let path = path.map(|path| path.as_path().join(PROOF));

        if path.as_ref().map_or(false, |p| p.exists()) {
            let mut fd = fs::File::create(path.unwrap()).unwrap();
            fd.write_all(&mut proof.clone()).unwrap()
        }
    }

    pub fn create_proof(
        &self,
        params: &Params<E::G1Affine>,
        pk: &ProvingKey<C>,
        public_inputs: &Vec<C::ScalarExt>,
        transcript: &mut W,
    ) -> Result<(), Error> {
        let timer = start_timer!(|| "create proof");
        create_proof(
            params,
            pk,
            &[self.circuit.clone()],
            &[&[public_inputs]],
            OsRng,
            transcript,
        )?;
        end_timer!(timer);

        Ok(())
    }

    pub fn verify_check(
        &self,
        vk: &VerifyingKey<C>,
        params: &Params<C>,
        public_inputs: &Vec<C::ScalarExt>,
        transcript: &mut R,
    ) {
        let public_inputs_size = public_inputs.len();

        let params_verifier: ParamsVerifier<E> = params.verifier(public_inputs_size).unwrap();

        let strategy = SingleVerifier::new(&params_verifier);

        let timer = start_timer!(|| "verify proof");
        verify_proof(
            &params_verifier,
            vk,
            strategy,
            &[&[public_inputs]],
            transcript,
        )
        .unwrap();
        end_timer!(timer);
    }
    /*
        pub fn run(
            &self,
            public_inputs: Vec<C::ScalarExt>,
            params_dir: Option<PathBuf>,
            vk_dir: Option<PathBuf>,
            proof_dir: Option<PathBuf>,
        ) -> (Params<C>, VerifyingKey<C>, Vec<u8>) {
            let params = self.prepare_param(params_dir);

            let vk = self.prepare_vk(&self.circuit, &params, vk_dir);
            let pk = self.prepare_pk(&self.circuit, &params, vk);

            let proof = {
                if let Some(proof) = self.try_load_proof(proof_dir) {
                    proof
                } else {
                    let mut transcript = Blake2bWrite::init(vec![]);
                    let proof = self.create_proof(&params, &pk, &public_inputs, transcript);
                    let proof = transcript.finalize();

                    self.try_save_proof(&proof, proof_dir);

                    proof
                }
            };

            let mut transcript = Blake2bRead::init(&proof[..]);
            self.verify_check(pk.get_vk(), &params, &public_inputs, transcript);

            (params, vk, proof)
        }
    */
    /*
        pub fn bench_with_result(
            &self,
            public_inputs: Vec<C::ScalarExt>,
        ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
            let circuit: TestCircuit<C::ScalarExt> = self.build_circuit::<C::ScalarExt>();

            let mut params_buffer: Vec<u8> = vec![];
            let params = self.create_params();
            params.write::<Vec<u8>>(params_buffer.borrow_mut()).unwrap();
            let vk = self.prepare_vk(&circuit, &params);

            let mut vk_buffer: Vec<u8> = vec![];
            vk.write::<Vec<u8>>(vk_buffer.borrow_mut()).unwrap();
            let pk = self.prepare_pk(&circuit, &params, vk);

            let proof = self.create_proof(&[circuit], &params, &pk, &public_inputs);
            self.verify_check(pk.get_vk(), &params, &proof, &public_inputs);

            (params_buffer, vk_buffer, proof)
        }
    */
}

fn clone_vk<C: CurveAffine>(params: &Params<C>, vk: &VerifyingKey<C>) -> VerifyingKey<C> {
    let mut buf = vec![];
    vk.write(&mut buf).unwrap();

    VerifyingKey::read::<_, TestCircuit<_>>(&mut Cursor::new(buf), params).unwrap()
}

pub fn run_circuit(
    compile_tables: CompileTable,
    execution_tables: ExecutionTable,
    public_inputs: Vec<Fr>,
) -> (Params<G1Affine>, VerifyingKey<G1Affine>, Vec<u8>) {
    let builder: ZkWasmCircuitBuilder<G1Affine, Bn256, _, _, _> =
        ZkWasmCircuitBuilder::new(compile_tables, execution_tables);

    let params: halo2_proofs::poly::commitment::Params<G1Affine> =
        builder.prepare_param(Some(PathBuf::from("./")));

    let vk = builder.prepare_vk(&params, None);
    let pk = builder.prepare_pk(&params, vk);

    let mut transcript = Blake2bWrite::init(vec![]);
    builder
        .create_proof(&params, &pk, &public_inputs, &mut transcript)
        .unwrap();
    let proof = transcript.finalize();

    let mut transcript = Blake2bRead::init(&proof[..]);
    builder.verify_check(pk.get_vk(), &params, &public_inputs, &mut transcript);

    let vk = clone_vk(&params, pk.get_vk());
    (params, vk, proof)
}
