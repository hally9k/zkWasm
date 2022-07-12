use super::utils::bn_to_field;
use super::utils::Context;
use super::Encode;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::plonk::Advice;
use halo2_proofs::plonk::Column;
use halo2_proofs::plonk::Error;
use num_bigint::BigUint;
use specs::jtable::JumpTableEntry;
use std::marker::PhantomData;

impl Encode for JumpTableEntry {
    fn encode(&self) -> BigUint {
        todo!()
    }
}

#[derive(Clone)]
pub struct JumpTableConfig<F: FieldExt> {
    col: Column<Advice>,
    _mark: PhantomData<F>,
}

impl<F: FieldExt> JumpTableConfig<F> {
    pub fn configure(
        cols: &mut impl Iterator<Item = Column<Advice>>,
    ) -> Self {
        Self {
            col: cols.next().unwrap(),
            _mark: PhantomData,
        }
    }
}

pub struct EventTableChip<F: FieldExt> {
    config: JumpTableConfig<F>,
}

impl<F: FieldExt> EventTableChip<F> {
    pub fn new(config: JumpTableConfig<F>) -> Self {
        EventTableChip { config }
    }

    pub fn add_jump(
        &self,
        ctx: &mut Context<'_, F>,
        jump: Box<JumpTableEntry>,
    ) -> Result<(), Error> {
        ctx.region.assign_advice_from_constant(
            || "jump table entry",
            self.config.col,
            ctx.offset,
            bn_to_field(&jump.encode()),
        )?;
        Ok(())
    }
}
