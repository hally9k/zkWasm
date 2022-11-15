use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{layouter::context::Context, nextn};

#[derive(Clone, Copy)]
pub struct U64OnU8Cell {
    pub value_col: Column<Advice>,
    pub value_rot: i32,
    pub u8_col: Column<Advice>,
    pub u8_rot: i32,
}

impl U64OnU8Cell {
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

        for i in 0..8usize {
            let v = value & 0xff;
            value >>= 8;
            ctx.region.assign_advice(
                || "u8 range cell",
                self.u8_col,
                ((ctx.offset + i) as i32 + self.u8_rot) as usize,
                || Ok(F::from(v)),
            )?;
        }

        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.value_col, self.value_rot)
    }

    pub fn u8_expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>, i: i32) -> Expression<F> {
        nextn!(meta, self.u8_col, i + self.u8_rot)
    }
}
