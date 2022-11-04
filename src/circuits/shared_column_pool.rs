use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};

use crate::{
    circuits::config::{ETABLE_END_OFFSET, ETABLE_START_OFFSET},
    curr, fixed_curr,
};

use super::{
    config::{MTABLE_END_OFFSET, MTABLE_START_OFFSET},
    rtable::RangeTableConfig,
};

const U8_COLUMNS: usize = 2;

#[derive(Clone)]
pub struct SharedColumnPool<F> {
    sel: Column<Fixed>,
    u8_col: [Column<Advice>; U8_COLUMNS],
    _mark: PhantomData<F>,
}

impl<F: FieldExt> SharedColumnPool<F> {
    pub fn configure(meta: &mut ConstraintSystem<F>, rtable: &RangeTableConfig<F>) -> Self {
        let sel = meta.fixed_column();
        let u8_col = [(); U8_COLUMNS].map(|_| meta.advice_column());

        for i in 0..U8_COLUMNS {
            rtable.configure_in_u8_range(meta, "mtable bytes", |meta| {
                curr!(meta, u8_col[i]) * fixed_curr!(meta, sel)
            });
        }

        SharedColumnPool::<F> {
            sel,
            u8_col,
            _mark: PhantomData,
        }
    }

    pub fn acquire_sel_col(&self) -> Column<Fixed> {
        self.sel.clone()
    }

    pub fn acquire_u8_col(&self, index: usize) -> Column<Advice> {
        self.u8_col[index].clone()
    }
}

pub struct SharedColumnChip<F> {
    config: SharedColumnPool<F>,
}

impl<F: FieldExt> SharedColumnChip<F> {
    pub fn new(config: SharedColumnPool<F>) -> Self {
        SharedColumnChip::<F> { config }
    }

    pub fn init(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "shared column",
            |mut region| {
                for o in ETABLE_START_OFFSET..ETABLE_END_OFFSET {
                    region.assign_fixed(
                        || "shared column sel",
                        self.config.sel,
                        o,
                        || Ok(F::from(1u64)),
                    )?;
                }

                for o in MTABLE_START_OFFSET..MTABLE_END_OFFSET {
                    region.assign_fixed(
                        || "shared column sel",
                        self.config.sel,
                        o,
                        || Ok(F::from(1u64)),
                    )?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}