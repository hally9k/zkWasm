use std::marker::PhantomData;

use halo2_proofs::plonk::{Advice, Column, Fixed};

pub mod opcode;
pub mod step_status;

pub const ETABLE_STEP_SIZE: usize = 20usize;
pub const U4_COLUMNS: usize = 3usize;
pub const U8_COLUMNS: usize = 2usize;
pub const BITS_COLUMNS: usize = 2usize;
pub const MTABLE_LOOKUPS_SIZE: usize = 6usize;
pub const MAX_OP_LVL1: i32 = (ETABLE_STEP_SIZE >> 1) as i32;
pub const MAX_OP_LVL2: i32 = ETABLE_STEP_SIZE as i32;

pub enum EventTableBitColumnRotation {
    Enable = 0,
    Max,
}

pub enum EventTableCommonRangeColumnRotation {
    RestMOps = 0,
    RestJOps,
    InputIndex,
    EID,
    MOID,
    FID,
    IID,
    MMID,
    SP,
    LastJumpEid,
    Max,
}

pub enum EventTableUnlimitColumnRotation {
    ITableLookup = 0,
    JTableLookup = 1,
    PowTableLookup = 2,
    OffsetLenBitsTableLookup = 3,
    MTableLookupStart = 4,
    U64Start = 5 + MTABLE_LOOKUPS_SIZE as isize,
}

#[derive(Clone)]
pub struct EventTableCommonConfig<F> {
    pub sel: Column<Fixed>,
    pub block_first_line_sel: Column<Fixed>,

    pub shared_bits: [Column<Advice>; BITS_COLUMNS],
    pub opcode_bits: Column<Advice>,

    pub state: Column<Advice>,

    pub unlimited: Column<Advice>,

    pub itable_lookup: Column<Fixed>,
    pub jtable_lookup: Column<Fixed>,
    pub mtable_lookup: Column<Fixed>,
    pub pow_table_lookup: Column<Fixed>,
    pub offset_len_bits_table_lookup: Column<Fixed>,

    pub aux: Column<Advice>,

    pub u4_bop: Column<Advice>,
    pub u4_shared: [Column<Advice>; U4_COLUMNS],
    pub u8_shared: [Column<Advice>; U8_COLUMNS],

    pub _mark: PhantomData<F>,
}
