use crate::{mtable::LocationType, types::ValueType};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct InitMemoryTableEntry {
    pub ltype: LocationType,
    pub is_mutable: bool,
    pub mmid: u64,
    pub offset: u64,
    pub vtype: ValueType,
    /// convert from [u8; 8] via u64::from_le_bytes
    pub value: u64,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct InitMemoryTable {
    entries: Vec<InitMemoryTableEntry>,
    finalized: bool,
}

impl InitMemoryTable {
    pub fn new() -> Self {
        InitMemoryTable {
            entries: vec![],
            finalized: false,
        }
    }

    pub fn push(&mut self, entry: &InitMemoryTableEntry) {
        self.entries.push(entry.clone())
    }

    pub fn finalized(&mut self) {
        self.sort();
        self.finalized = true;
    }

    pub fn entries(&self) -> &Vec<InitMemoryTableEntry> {
        assert!(self.finalized);

        &self.entries
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.entries).unwrap()
    }

    pub fn find(&self, ltype: LocationType, mmid: u64, offset: u64) -> u64 {
        for entry in self.entries.iter() {
            if entry.ltype == ltype && entry.mmid == mmid && entry.offset == offset {
                return entry.value;
            }
        }

        unreachable!()
    }

    fn sort(&mut self) {
        self.entries
            .sort_by_key(|item| (item.ltype, item.mmid, item.offset))
    }

    pub fn filter(&self, ltype: LocationType) -> Vec<&InitMemoryTableEntry> {
        assert!(self.finalized);

        self.entries.iter().filter(|e| e.ltype == ltype).collect()
    }
}
