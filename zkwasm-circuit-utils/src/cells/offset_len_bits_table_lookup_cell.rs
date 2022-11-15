use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Error, Expression, VirtualCells},
};

use crate::{constant_from, layouter::context::Context, nextn};

pub fn bits_of_offset_len(offset: u64, len: u64) -> u64 {
    let bits = (1 << len) - 1;
    bits << offset
}

pub fn offset_len_bits_encode(offset: u64, len: u64) -> u64 {
    assert!(offset < 16);
    assert!(len == 1 || len == 2 || len == 4 || len == 8);
    (offset << 20) + (len << 16) + bits_of_offset_len(offset, len)
}

pub fn offset_len_bits_encode_expr<F: FieldExt>(
    offset: Expression<F>,
    len: Expression<F>,
    bits: Expression<F>,
) -> Expression<F> {
    offset * constant_from!(1u64 << 20) + len * constant_from!(1u64 << 16) + bits
}

#[derive(Copy, Clone)]
pub struct OffsetLenBitsTableLookupCell {
    pub col: Column<Advice>,
    pub rot: i32,
}

impl OffsetLenBitsTableLookupCell {
    pub fn assign<F: FieldExt>(
        &self,
        ctx: &mut Context<'_, F>,
        offset: u64,
        len: u64,
    ) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "offset len bits lookup cell",
            self.col,
            (ctx.offset as i32 + self.rot) as usize,
            || Ok(F::from(offset_len_bits_encode(offset, len))),
        )?;
        Ok(())
    }

    pub fn expr<F: FieldExt>(&self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        nextn!(meta, self.col, self.rot)
    }
}
