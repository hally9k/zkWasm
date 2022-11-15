use super::*;
use halo2_proofs::arithmetic::FieldExt;
use zkwasm_circuit_utils::{
    cells::{
        bit_cell::BitCell, common_range_cell::CommonRangeCell,
        jtable_lookup_cell::JTableLookupCell, mtable_lookup_cell::MTableLookupCell,
        offset_len_bits_table_lookup_cell::OffsetLenBitsTableLookupCell,
        pow_table_lookup_cell::PowTableLookupCell, u4_bop_cell::U4BopCell, u64_cell::U64Cell,
        u64_on_u8_cell::U64OnU8Cell, unlimited_cell::UnlimitedCell,
    },
    layouter::context::Context,
    tables::etable::configure::EventTableCommonConfig,
};
use zkwasm_types::itable::OpcodeClass;

pub(super) mod op_bin;
pub(super) mod op_bin_bit;
pub(super) mod op_bin_shift;
pub(super) mod op_br;
pub(super) mod op_br_if;
pub(super) mod op_br_if_eqz;
pub(super) mod op_call;
pub(super) mod op_const;
pub(super) mod op_conversion;
pub(super) mod op_drop;
pub(super) mod op_global_get;
pub(super) mod op_global_set;
pub(super) mod op_load;
pub(super) mod op_local_get;
pub(super) mod op_local_set;
pub(super) mod op_local_tee;
pub(super) mod op_rel;
pub(super) mod op_return;
pub(crate) mod op_select;
pub(super) mod op_store;
pub(super) mod op_test;

pub trait EventTableOpcodeConfigBuilder<F: FieldExt> {
    fn configure(
        common: &mut EventTableCellAllocator<F>,
        constraint_builder: &mut ConstraintBuilder<F>,
    ) -> Box<dyn EventTableOpcodeConfig<F>>;
}

pub trait EventTableForeignOpcodeConfigBuilder<F: FieldExt> {
    fn configure(
        common: &mut EventTableCellAllocator<F>,
        constraint_builder: &mut ConstraintBuilder<F>,
    ) -> Box<dyn EventTableOpcodeConfig<F>>;
}
