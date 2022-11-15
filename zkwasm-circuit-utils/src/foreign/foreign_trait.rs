use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{ConstraintSystem, Expression, VirtualCells},
};

pub trait ForeignTableConfig<F: FieldExt> {
    fn configure_in_table(
        &self,
        meta: &mut ConstraintSystem<F>,
        key: &'static str,
        expr: &dyn Fn(&mut VirtualCells<'_, F>) -> Expression<F>,
    );
}

pub trait ForeignCallInfo {
    fn call_id(&self) -> usize;
}
