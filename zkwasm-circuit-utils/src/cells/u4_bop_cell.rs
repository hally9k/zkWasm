use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{constant_from, layouter::context::Context, nextn};
#[derive(Clone, Copy)]
pub struct U4BopCell {
    pub col: Column<Advice>,
}

impl U4BopCell {
    pub fn assign<F: FieldExt>(&self, ctx: &mut Context<'_, F>, value: F) -> Result<(), Error> {
        for i in 0..16usize {
            ctx.region.assign_advice(
                || "u4 bop cell",
                self.col,
                ctx.offset + i,
                || Ok(F::from(value)),
            )?;
        }

        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, 0)
    }

    pub fn eq_constraint<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        let mut sum = constant_from!(0);
        for i in 1..16 {
            sum = sum + nextn!(meta, self.col, i);
        }
        sum - constant_from!(15) * nextn!(meta, self.col, 0)
    }
}
