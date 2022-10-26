use std::{fs::File, io::Write, path::PathBuf};

use halo2_proofs::{
    pairing::bn256::{Bn256, Fr, G1Affine},
    plonk::VerifyingKey,
    poly::commitment::Params,
};
use halo2_snark_aggregator_solidity::MultiCircuitSolidityGenerate;

pub(crate) struct SolidityVerifier;

impl SolidityVerifier {
    pub(crate) fn generate_verifier<'a>(
        verify_params: &'a Params<G1Affine>,
        verify_vk: &'a VerifyingKey<G1Affine>,
        proof: Vec<u8>,
        public_inputs: Vec<Fr>,
    ) {
        let generator = MultiCircuitSolidityGenerate::<_, 1> {
            verify_params,
            verify_vk,
            proof,
            verify_circuit_instance: vec![vec![public_inputs.clone()]],
            verify_public_inputs_size: public_inputs.len(),
        };

        let template_folder =
            PathBuf::from("../scroll-halo2-verifier/halo2-snark-aggregator-solidity/templates");

        let sss = generator.call::<Bn256>(template_folder);

        println!("{:?}", sss);

        let sol_path = PathBuf::from("./verifier.sol");

        let mut output = File::create(sol_path).unwrap();
        write!(output, "{}", sss).unwrap();

        ()
    }
}
