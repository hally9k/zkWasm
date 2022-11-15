use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{layouter::context::Context, nextn};

#[derive(Copy, Clone)]
pub struct UnlimitedCell {
    pub col: Column<Advice>,
    pub rot: i32,
}

impl UnlimitedCell {
    pub fn assign<F: FieldExt>(&self, ctx: &mut Context<'_, F>, value: F) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "cell",
            self.col,
            (ctx.offset as i32 + self.rot) as usize,
            || Ok(value),
        )?;
        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, self.rot)
    }
}
