use super::{etable_compact::ETABLE_STEP_SIZE, mtable_compact::configure::MTABLE_STEP_SIZE};

pub const K: u32 = 20;
const ETABLE_SLICE_ROWS: usize = 1usize << (K - 2);
const MTABLE_SLICE_ROWS: usize = 1usize << (K - 1);
pub const MAX_JATBLE_ROWS: usize = 1usize << (K - 6);
pub const IMTABLE_COLOMNS: usize = 2;

pub const POW_TABLE_LIMIT: u64 = 128;

pub const MAX_ETABLE_ROWS: usize = ETABLE_SLICE_ROWS / ETABLE_STEP_SIZE * ETABLE_STEP_SIZE;
pub const MAX_MTABLE_ROWS: usize =
    MTABLE_SLICE_ROWS / MTABLE_STEP_SIZE as usize * MTABLE_STEP_SIZE as usize;

pub const ETABLE_START_OFFSET: usize = 0;
pub const ETABLE_END_OFFSET: usize = ETABLE_START_OFFSET + MAX_ETABLE_ROWS;
pub const MTABLE_START_OFFSET: usize = 1usize << (K - 2);
pub const MTABLE_END_OFFSET: usize = MTABLE_START_OFFSET + MAX_MTABLE_ROWS;
pub const JTABLE_START_OFFSET: usize = (1usize << (K - 2)) * 3;
pub const JTABLE_END_OFFSET: usize = JTABLE_START_OFFSET + MAX_JATBLE_ROWS;
