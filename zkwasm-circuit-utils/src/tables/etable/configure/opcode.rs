use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Error, Expression, VirtualCells},
};
use zkwasm_types::{etable::EventTableEntry, itable::OpcodeClass};

use super::{step_status::StepStatus, EventTableCommonConfig};
use crate::layouter::context::Context;

pub enum MLookupItem {
    First = 0,
    Second,
    Third,
    Fourth,
    Fifth,
    Six,
}

impl From<usize> for MLookupItem {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::First,
            1 => Self::Second,
            2 => Self::Third,
            3 => Self::Fourth,
            4 => Self::Fifth,
            5 => Self::Six,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u32> for MLookupItem {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::First),
            1 => Ok(Self::Second),
            2 => Ok(Self::Third),
            3 => Ok(Self::Fourth),
            _ => unreachable!(),
        }
    }
}

pub trait EventTableOpcodeConfig<F: FieldExt> {
    fn opcode(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F>;
    fn opcode_class(&self) -> OpcodeClass;

    fn assign(
        &self,
        ctx: &mut Context<'_, F>,
        step: &StepStatus,
        entry: &EventTableEntry,
    ) -> Result<(), Error>;

    fn assigned_extra_mops(
        &self,
        _ctx: &mut Context<'_, F>,
        _step: &StepStatus,
        _entry: &EventTableEntry,
    ) -> u64 {
        0u64
    }

    fn sp_diff(&self, _meta: &mut VirtualCells<'_, F>) -> Option<Expression<F>> {
        None
    }

    fn jops(&self, _meta: &mut VirtualCells<'_, F>) -> Option<Expression<F>> {
        None
    }
    fn mops(&self, _meta: &mut VirtualCells<'_, F>) -> Option<Expression<F>> {
        None
    }
    fn next_last_jump_eid(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn next_moid(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn next_fid(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn next_iid(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn mtable_lookup(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _item: MLookupItem,
        _common: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn jtable_lookup(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn itable_lookup(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn input_index_increase(
        &self,
        _meta: &mut VirtualCells<'_, F>,
        _common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        None
    }
    fn is_host_public_input(&self, _step: &StepStatus, _entry: &EventTableEntry) -> bool {
        false
    }
}
