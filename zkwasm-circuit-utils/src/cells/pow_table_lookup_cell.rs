use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};
use num_bigint::BigUint;
use zkwasm_config::POW_TABLE_LIMIT;

use crate::{expr::bn_to_field, layouter::context::Context, nextn};

#[derive(Copy, Clone)]
pub struct PowTableLookupCell {
    pub col: Column<Advice>,
    pub rot: i32,
}

impl PowTableLookupCell {
    pub fn assign<F: FieldExt>(&self, ctx: &mut Context<'_, F>, power: u64) -> Result<(), Error> {
        assert!(power < POW_TABLE_LIMIT);
        ctx.region.assign_advice(
            || "pow lookup cell",
            self.col,
            (ctx.offset as i32 + self.rot) as usize,
            || {
                Ok(bn_to_field(
                    &((BigUint::from(1u64) << (power + 16)) + power),
                ))
            },
        )?;
        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, self.rot)
    }
}
