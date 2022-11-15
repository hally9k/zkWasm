use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};
use num_bigint::BigUint;

use crate::{expr::bn_to_field, layouter::context::Context, nextn};

pub struct JTableLookupCell {
    pub col: Column<Advice>,
    pub rot: i32,
}

impl JTableLookupCell {
    pub fn assign<F: FieldExt>(
        &self,
        ctx: &mut Context<'_, F>,
        value: &BigUint,
    ) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "jlookup cell",
            self.col,
            (ctx.offset as i32 + self.rot) as usize,
            || Ok(bn_to_field(value)),
        )?;
        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, self.rot)
    }
}
