use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Layouter,
    plonk::{ConstraintSystem, Error, Expression, TableColumn, VirtualCells},
};
use num_bigint::BigUint;
use num_traits::identities::Zero;
use num_traits::One;
use std::marker::PhantomData;
use zkwasm_circuit_utils::{constant, expr::bn_to_field};
use zkwasm_types::itable::InstructionTableEntry;

pub trait Encode {
    fn encode(&self) -> BigUint;
    fn encode_addr(&self) -> BigUint;
}

impl Encode for InstructionTableEntry {
    fn encode(&self) -> BigUint {
        let opcode: BigUint = self.opcode.clone().into();
        let mut bn = self.encode_addr();
        bn <<= 128usize;
        bn += opcode;
        bn
    }

    fn encode_addr(&self) -> BigUint {
        let mut bn = BigUint::zero();
        bn += self.moid;
        bn <<= 16u8;
        bn += self.mmid;
        bn <<= 16u8;
        bn += self.fid;
        bn <<= 16u8;
        bn += self.iid;
        bn
    }
}

pub fn encode_inst_expr<F: FieldExt>(
    moid: Expression<F>,
    mmid: Expression<F>,
    fid: Expression<F>,
    iid: Expression<F>,
    opcode: Expression<F>,
) -> Expression<F> {
    let mut bn = BigUint::one();
    let mut acc = opcode;
    bn <<= 128u8;
    acc = acc + iid * constant!(bn_to_field(&bn));
    bn <<= 16u8;
    acc = acc + fid * constant!(bn_to_field(&bn));
    bn <<= 16u8;
    acc = acc + mmid * constant!(bn_to_field(&bn));
    bn <<= 16u8;
    acc = acc + moid * constant!(bn_to_field(&bn));

    acc
}

#[derive(Clone)]
pub struct InstructionTableConfig<F: FieldExt> {
    col: TableColumn,
    _mark: PhantomData<F>,
}

impl<F: FieldExt> InstructionTableConfig<F> {
    pub fn configure(col: TableColumn) -> Self {
        InstructionTableConfig {
            col,
            _mark: PhantomData,
        }
    }

    pub fn configure_in_table(
        &self,
        meta: &mut ConstraintSystem<F>,
        key: &'static str,
        expr: impl FnOnce(&mut VirtualCells<'_, F>) -> Expression<F>,
    ) {
        meta.lookup(key, |meta| vec![(expr(meta), self.col)]);
    }
}

#[derive(Clone)]
pub struct InstructionTableChip<F: FieldExt> {
    config: InstructionTableConfig<F>,
}

impl<F: FieldExt> InstructionTableChip<F> {
    pub fn new(config: InstructionTableConfig<F>) -> Self {
        InstructionTableChip { config }
    }

    pub fn assign(
        self,
        layouter: &mut impl Layouter<F>,
        instructions: &Vec<InstructionTableEntry>,
    ) -> Result<(), Error> {
        layouter.assign_table(
            || "itable",
            |mut table| {
                table.assign_cell(|| "inst_init table", self.config.col, 0, || Ok(F::zero()))?;
                for (i, v) in instructions.iter().enumerate() {
                    table.assign_cell(
                        || "inst_init table",
                        self.config.col,
                        i + 1,
                        || Ok(bn_to_field::<F>(&v.encode())),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
