use halo2_proofs::arithmetic::FieldExt;

use crate::{
    cells::{
        bit_cell::BitCell, common_range_cell::CommonRangeCell,
        jtable_lookup_cell::JTableLookupCell, mtable_lookup_cell::MTableLookupCell,
        offset_len_bits_table_lookup_cell::OffsetLenBitsTableLookupCell,
        pow_table_lookup_cell::PowTableLookupCell, u4_bop_cell::U4BopCell, u64_cell::U64Cell,
        u64_on_u8_cell::U64OnU8Cell, unlimited_cell::UnlimitedCell,
    },
    tables::etable::configure::{BITS_COLUMNS, ETABLE_STEP_SIZE, U4_COLUMNS, U8_COLUMNS},
};

use super::configure::{
    EventTableBitColumnRotation, EventTableCommonConfig, EventTableCommonRangeColumnRotation,
    EventTableUnlimitColumnRotation,
};

pub struct EventTableCellAllocator<'a, F> {
    pub config: &'a EventTableCommonConfig<F>,
    pub bit_index: i32,
    pub common_range_index: i32,
    pub unlimited_index: i32,
    pub u4_bop_index: i32,
    pub u64_index: i32,
    pub u64_on_u8_index: i32,
    pub mtable_lookup_index: i32,
    pub jtable_lookup_index: i32,
    pub pow_table_lookup_index: i32,
    pub offset_len_bits_lookup_index: i32,
}

impl<'a, F: FieldExt> EventTableCellAllocator<'a, F> {
    pub fn new(config: &'a EventTableCommonConfig<F>) -> Self {
        Self {
            config,
            bit_index: EventTableBitColumnRotation::Max as i32,
            common_range_index: EventTableCommonRangeColumnRotation::Max as i32,
            unlimited_index: 0,
            u4_bop_index: 0,
            u64_index: 0,
            u64_on_u8_index: 0,
            pow_table_lookup_index: EventTableUnlimitColumnRotation::PowTableLookup as i32,
            mtable_lookup_index: EventTableUnlimitColumnRotation::MTableLookupStart as i32,
            jtable_lookup_index: EventTableUnlimitColumnRotation::JTableLookup as i32,
            offset_len_bits_lookup_index: EventTableUnlimitColumnRotation::OffsetLenBitsTableLookup
                as i32,
        }
    }

    pub fn alloc_bit_value(&mut self) -> BitCell {
        assert!(self.bit_index < BITS_COLUMNS as i32 * ETABLE_STEP_SIZE as i32);
        let allocated_index = self.bit_index;
        self.bit_index += 1;
        BitCell {
            col: self.config.shared_bits[allocated_index as usize / ETABLE_STEP_SIZE as usize],
            rot: allocated_index % ETABLE_STEP_SIZE as i32,
        }
    }

    pub fn alloc_common_range_value(&mut self) -> CommonRangeCell {
        assert!(self.common_range_index < ETABLE_STEP_SIZE as i32);
        let allocated_index = self.common_range_index;
        self.common_range_index += 1;
        CommonRangeCell {
            col: self.config.state,
            rot: allocated_index,
        }
    }

    pub fn alloc_unlimited_value(&mut self) -> UnlimitedCell {
        assert!(self.unlimited_index < ETABLE_STEP_SIZE as i32);
        let allocated_index = self.unlimited_index;
        self.unlimited_index += 1;
        UnlimitedCell {
            col: self.config.unlimited,
            rot: allocated_index,
        }
    }

    pub fn alloc_u4_bop(&mut self) -> U4BopCell {
        assert!(self.u4_bop_index < 1 as i32);
        self.u4_bop_index += 1;
        U4BopCell {
            col: self.config.u4_bop,
        }
    }

    pub fn alloc_u64(&mut self) -> U64Cell {
        assert!(self.u64_index < U4_COLUMNS as i32);
        let allocated_index = self.u64_index;
        self.u64_index += 1;
        U64Cell {
            value_col: self.config.aux,
            value_rot: allocated_index + EventTableUnlimitColumnRotation::U64Start as i32,
            u4_col: self.config.u4_shared[allocated_index as usize],
        }
    }

    pub fn alloc_u64_on_u8(&mut self) -> U64OnU8Cell {
        assert!(self.u64_on_u8_index < U8_COLUMNS as i32 * 2);
        let allocated_index = self.u64_on_u8_index;
        self.u64_on_u8_index += 1;
        U64OnU8Cell {
            value_col: self.config.aux,
            value_rot: allocated_index
                + EventTableUnlimitColumnRotation::U64Start as i32
                + U4_COLUMNS as i32,
            u8_col: self.config.u8_shared[allocated_index as usize / 2],
            u8_rot: (allocated_index % 2) * 8,
        }
    }

    pub fn alloc_mtable_lookup(&mut self) -> MTableLookupCell {
        assert!(self.mtable_lookup_index < EventTableUnlimitColumnRotation::U64Start as i32);
        let allocated_index = self.mtable_lookup_index;
        self.mtable_lookup_index += 1;
        MTableLookupCell {
            col: self.config.aux,
            rot: allocated_index,
        }
    }

    pub fn alloc_pow_table_lookup(&mut self) -> PowTableLookupCell {
        assert!(
            self.pow_table_lookup_index
                < EventTableUnlimitColumnRotation::OffsetLenBitsTableLookup as i32
        );
        let allocated_index = self.pow_table_lookup_index;
        self.pow_table_lookup_index += 1;
        PowTableLookupCell {
            col: self.config.aux,
            rot: allocated_index,
        }
    }

    pub fn alloc_offset_len_bits_table_lookup(&mut self) -> OffsetLenBitsTableLookupCell {
        assert!(
            self.offset_len_bits_lookup_index
                < EventTableUnlimitColumnRotation::MTableLookupStart as i32
        );
        let allocated_index = self.offset_len_bits_lookup_index;
        self.offset_len_bits_lookup_index += 1;
        OffsetLenBitsTableLookupCell {
            col: self.config.aux,
            rot: allocated_index,
        }
    }

    pub fn alloc_jtable_lookup(&mut self) -> JTableLookupCell {
        assert!(
            self.jtable_lookup_index < EventTableUnlimitColumnRotation::MTableLookupStart as i32
        );
        let allocated_index = self.jtable_lookup_index;
        self.jtable_lookup_index += 1;
        JTableLookupCell {
            col: self.config.aux,
            rot: allocated_index,
        }
    }

    pub fn input_index_cell(&self) -> UnlimitedCell {
        UnlimitedCell {
            col: self.config.state.clone(),
            rot: EventTableCommonRangeColumnRotation::InputIndex as i32,
        }
    }

    pub fn moid_cell(&self) -> UnlimitedCell {
        UnlimitedCell {
            col: self.config.state.clone(),
            rot: EventTableCommonRangeColumnRotation::MOID as i32,
        }
    }
}
