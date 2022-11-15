use super::itable::InstructionTableEntry;
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct FrameTableEntry {
    // caller eid (unique)
    pub eid: u64,
    pub last_jump_eid: u64,
    pub inst: Box<InstructionTableEntry>,
}

impl FrameTableEntry {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn encode(&self) -> BigUint {
        let mut bn = BigUint::from(self.eid);
        bn = bn << 16;
        bn += self.last_jump_eid;
        bn = bn << 48;
        bn += self.inst.encode_instruction_address();
        bn
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FrameTable(Vec<FrameTableEntry>);

impl FrameTable {
    pub fn new() -> Self {
        FrameTable(vec![])
    }

    pub fn push(&mut self, entry: &FrameTableEntry) {
        self.0.push(entry.clone())
    }

    pub fn entries(&self) -> &Vec<FrameTableEntry> {
        &self.0
    }
}
