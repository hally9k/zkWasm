#[cfg(test)]
mod tests {
    use crate::circuits::run_circuit;
    use crate::test::test_binary_search::build_test;
    use halo2_proofs::pairing::bn256::Fr as Fp;

    #[test]
    fn test_binary_search() {
        let (compiled_module, execution_log, public_inputs) = build_test();

        let _ = run_circuit(
            compiled_module.tables,
            execution_log.tables,
            public_inputs.into_iter().map(|v| Fp::from(v)).collect(),
        );
    }
}
