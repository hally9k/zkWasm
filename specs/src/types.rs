use serde::Serialize;
use strum_macros::EnumIter;

use crate::host_function::HostPlugin;

#[derive(Clone, Copy, Debug, PartialEq, EnumIter, Serialize, Hash, Eq)]
pub enum ValueType {
    I32 = 1,
    I64 = 2,
}

impl ValueType {
    pub fn byte_size(&self) -> u64 {
        match self {
            ValueType::I32 => 4,
            ValueType::I64 => 8,
        }
    }
}

impl From<parity_wasm::elements::ValueType> for ValueType {
    fn from(v: parity_wasm::elements::ValueType) -> Self {
        match v {
            parity_wasm::elements::ValueType::I32 => ValueType::I32,
            parity_wasm::elements::ValueType::I64 => ValueType::I64,
            parity_wasm::elements::ValueType::F32 => todo!(),
            parity_wasm::elements::ValueType::F64 => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl Into<ValueType> for Value {
    fn into(self) -> ValueType {
        match self {
            Value::I32(_) => ValueType::I32,
            Value::I64(_) => ValueType::I64,
        }
    }
}

impl Value {
    pub fn internal(&self) -> u64 {
        match self {
            Value::I32(v) => (*v) as u64,
            Value::I64(v) => (*v) as u64,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum FunctionType {
    WasmFunction,
    HostFunction {
        plugin: HostPlugin,
        function_index: usize,
        function_name: String,
        op_index_in_plugin: usize,
    },
}

#[derive(Debug)]
pub enum CompileError {}

#[derive(Debug)]
pub enum ExecutionError {}
