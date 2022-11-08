use crate::circuits::ZkWasmCircuitBuilder;
use crate::test::test_binary_search::build_test;

pub fn build_circuit() -> ZkWasmCircuitBuilder {
        let (compiled_module, execution_log, public_inputs) = build_test();

        let builder = ZkWasmCircuitBuilder {
            compile_tables: compiled_module.tables,
            execution_tables: execution_log.tables,
        };

        builder
}


#[cfg(test)]
mod tests {
    use crate::circuits::ZkWasmCircuitBuilder;
    use crate::test::test_binary_search::build_test;
    use halo2_proofs::pairing::bn256::Fr as Fp;

    #[test]
    fn test_binary_search() {
        let (compiled_module, execution_log, public_inputs) = build_test();

        let builder = ZkWasmCircuitBuilder {
            compile_tables: compiled_module.tables,
            execution_tables: execution_log.tables,
        };

        builder.bench(public_inputs.into_iter().map(|v| Fp::from(v)).collect())
    }
}
