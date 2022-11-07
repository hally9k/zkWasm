use crate::{
    circuits::ZkWasmCircuitBuilder,
    foreign::{
        sha256_helper::runtime::register_sha256_foreign,
        wasm_input_helper::runtime::register_wasm_input_foreign,
    },
    runtime::{host::HostEnv, WasmInterpreter, WasmRuntime},
};

use std::{fs::File, io::Read, path::PathBuf};
use wasmi::ImportsBuilder;

pub fn build_circuit() -> (ZkWasmCircuitBuilder, Vec<u64>) {
    let public_inputs = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let private_inputs = vec![];

    let mut binary = vec![];

    let path = PathBuf::from("wasm/multi_instances.wasm");
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut binary).unwrap();

    let compiler = WasmInterpreter::new();

    let mut env = HostEnv::new();
    register_wasm_input_foreign(&mut env, public_inputs.clone(), private_inputs.clone());
    register_sha256_foreign(&mut env);
    let imports = ImportsBuilder::new().with_resolver("env", &env);

    let compiled_module = compiler
        .compile(&binary, &imports, &env.function_plugin_lookup)
        .unwrap();
    let execution_log = compiler
        .run(
            &mut env,
            &compiled_module,
            "multi_public_inputs",
            public_inputs.clone(),
            private_inputs,
        )
        .unwrap();

    (
        ZkWasmCircuitBuilder {
            compile_tables: compiled_module.tables,
            execution_tables: execution_log.tables,
        },
        public_inputs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::pairing::bn256::Fr as Fp;

    #[test]
    fn test_multi_instances() {
        let (builder, public_inputs) = build_circuit();

        builder.bench(public_inputs.into_iter().map(|v| Fp::from(v)).collect())
    }
}
