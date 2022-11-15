use super::*;
use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Error, Expression, VirtualCells},
};
use zkwasm_circuit_utils::{
    constant, constant_from,
    expr::bn_to_field,
    tables::{
        etable::configure::{opcode::MLookupItem, step_status::StepStatus},
        mtable::encode::MemoryTableLookupEncode,
    },
};
use zkwasm_types::{
    itable::{OPCODE_ARG0_SHIFT, OPCODE_CLASS_SHIFT},
    types::ValueType,
};

pub struct ConstConfig {
    vtype: CommonRangeCell,
    value: U64Cell,
    lookup_stack_write: MTableLookupCell,
}

pub struct ConstConfigBuilder {}

impl<F: FieldExt> EventTableOpcodeConfigBuilder<F> for ConstConfigBuilder {
    fn configure(
        common: &mut EventTableCellAllocator<F>,
        _constraint_builder: &mut ConstraintBuilder<F>,
    ) -> Box<dyn EventTableOpcodeConfig<F>> {
        let vtype = common.alloc_common_range_value();
        let value = common.alloc_u64();
        let lookup_stack_write = common.alloc_mtable_lookup();

        Box::new(ConstConfig {
            vtype,
            value,
            lookup_stack_write,
        })
    }
}

impl<F: FieldExt> EventTableOpcodeConfig<F> for ConstConfig {
    fn opcode(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        constant!(bn_to_field(
            &(BigUint::from(OpcodeClass::Const as u64) << OPCODE_CLASS_SHIFT)
        )) + self.vtype.expr(meta)
            * constant!(bn_to_field(&(BigUint::from(1u64) << OPCODE_ARG0_SHIFT)))
            + self.value.expr(meta)
    }

    fn assign(
        &self,
        ctx: &mut Context<'_, F>,
        step_info: &StepStatus,
        entry: &EventTableEntry,
    ) -> Result<(), Error> {
        match &entry.step_info {
            zkwasm_types::step::StepInfo::I32Const { value } => {
                self.value.assign(ctx, *value as u32 as u64)?;
                self.vtype.assign(ctx, ValueType::I32 as u16)?;

                self.lookup_stack_write.assign(
                    ctx,
                    &MemoryTableLookupEncode::encode_stack_write(
                        BigUint::from(step_info.current.eid),
                        BigUint::from(1 as u64),
                        BigUint::from(step_info.current.sp),
                        BigUint::from(ValueType::I32 as u16),
                        BigUint::from(*value as u32 as u64),
                    ),
                )?;

                Ok(())
            }
            zkwasm_types::step::StepInfo::I64Const { value } => {
                self.value.assign(ctx, *value as u64)?;
                self.vtype.assign(ctx, ValueType::I64 as u16)?;

                self.lookup_stack_write.assign(
                    ctx,
                    &MemoryTableLookupEncode::encode_stack_write(
                        BigUint::from(step_info.current.eid),
                        BigUint::from(1 as u64),
                        BigUint::from(step_info.current.sp),
                        BigUint::from(ValueType::I64 as u16),
                        BigUint::from(*value as u64),
                    ),
                )?;

                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn opcode_class(&self) -> OpcodeClass {
        OpcodeClass::Const
    }

    fn sp_diff(&self, _meta: &mut VirtualCells<'_, F>) -> Option<Expression<F>> {
        Some(constant!(-F::one()))
    }

    fn mops(&self, _meta: &mut VirtualCells<'_, F>) -> Option<Expression<F>> {
        Some(constant_from!(1))
    }

    fn mtable_lookup(
        &self,
        meta: &mut VirtualCells<'_, F>,
        item: MLookupItem,
        common_config: &EventTableCommonConfig<F>,
    ) -> Option<Expression<F>> {
        match item {
            MLookupItem::First => Some(MemoryTableLookupEncode::encode_stack_write(
                common_config.eid(meta),
                constant_from!(1),
                common_config.sp(meta),
                self.vtype.expr(meta),
                self.value.expr(meta),
            )),
            MLookupItem::Second => None,
            MLookupItem::Third => None,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_circuit_noexternal;

    #[test]
    fn test_op_const_ok() {
        let textual_repr = r#"
                (module
                    (func (export "test")
                      (i32.const 0)
                      (drop)
                    )
                   )
                "#;

        test_circuit_noexternal(textual_repr).unwrap();
    }
}
