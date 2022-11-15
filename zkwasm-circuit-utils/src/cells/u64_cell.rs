use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{layouter::context::Context, nextn};

#[derive(Clone, Copy)]
pub struct U64Cell {
    pub value_col: Column<Advice>,
    pub value_rot: i32,
    pub u4_col: Column<Advice>,
}

impl U64Cell {
    pub fn assign<F: FieldExt>(
        &self,
        ctx: &mut Context<'_, F>,
        mut value: u64,
    ) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "u64 range cell",
            self.value_col,
            (ctx.offset as i32 + self.value_rot) as usize,
            || Ok(F::from(value)),
        )?;

        for i in 0..16usize {
            let v = value & 0xf;
            value >>= 4;
            ctx.region.assign_advice(
                || "u4 range cell",
                self.u4_col,
                ctx.offset + i,
                || Ok(F::from(v)),
            )?;
        }

        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.value_col, self.value_rot)
    }

    pub fn u4_expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>, i: i32) -> Expression<F> {
        nextn!(meta, self.u4_col, i)
    }
}
