use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{layouter::context::Context, nextn};

#[derive(Copy, Clone)]
pub struct BitCell {
    pub col: Column<Advice>,
    pub rot: i32,
}

impl BitCell {
    pub fn assign<F: FieldExt>(&self, ctx: &mut Context<'_, F>, value: bool) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "bit cell",
            self.col,
            (ctx.offset as i32 + self.rot) as usize,
            || Ok(F::from(value as u64)),
        )?;

        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, self.rot)
    }
}
