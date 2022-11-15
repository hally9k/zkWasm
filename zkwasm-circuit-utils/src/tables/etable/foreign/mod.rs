use halo2_proofs::arithmetic::FieldExt;

use crate::foreign::foreign_trait::ForeignCallInfo;

use super::{
    cell_allocator::EventTableCellAllocator, configure::opcode::EventTableOpcodeConfig,
    constraint_builder::ConstraintBuilder,
};

pub trait EventTableForeignCallConfigBuilder<F: FieldExt> {
    fn configure(
        common: &mut EventTableCellAllocator<F>,
        constraint_builder: &mut ConstraintBuilder<F>,
        info: &impl ForeignCallInfo,
    ) -> Box<dyn EventTableOpcodeConfig<F>>;
}
