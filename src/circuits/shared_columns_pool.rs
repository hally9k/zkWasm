use std::{array::IntoIter, marker::PhantomData};

use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Fixed, VirtualCells},
    poly::Rotation,
};

use crate::{constant, constant_from};

use super::{etable_compact::ETABLE_STEP_SIZE, utils::Context};

const SHARED_ADVICE_COLUMN: usize = 12;

#[derive(Copy, Clone)]
pub enum SharedColumnTableSelector {
    ExecutionTable = 1,
    MemoryTable = 2,
    FrameTable = 3,
}

#[derive(Clone)]
pub struct TableSelectorColumn<F> {
    internal: Column<Fixed>,
    _mark: PhantomData<F>,
}

impl<F: FieldExt> TableSelectorColumn<F> {
    fn alloc(meta: &mut ConstraintSystem<F>) -> Self {
        TableSelectorColumn {
            internal: meta.fixed_column(),
            _mark: PhantomData,
        }
    }

    pub fn assign(
        &self,
        ctx: &mut Context<'_, F>,
        value: SharedColumnTableSelector,
    ) -> Result<(), Error> {
        ctx.region.as_ref().borrow_mut().assign_fixed(
            || "table selector",
            self.internal,
            (ctx.offset as i32) as usize,
            || Ok(F::from(value as u64)),
        )?;

        Ok(())
    }

    pub fn expr(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        self.expr_rot(meta, Rotation::cur())
    }

    fn expr_rot(&self, meta: &mut VirtualCells<F>, rotation: Rotation) -> Expression<F> {
        meta.query_fixed(self.internal, rotation)
    }

    pub fn is_enable_mtable_entry(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        /*
         * First use as selector to avoid Poison, pick mtable entry
         */
        self.expr(meta)
            * (self.expr(meta) - constant_from!(SharedColumnTableSelector::ExecutionTable as u64))
            * (self.expr(meta) - constant_from!(SharedColumnTableSelector::FrameTable as u64))
    }

    pub fn is_enable_mtable_entry_bit(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        /*
         * Normalize
         */
        self.is_enable_mtable_entry(meta) * constant!(F::from(2).neg().invert().unwrap())
    }

    pub fn is_enable_jtable_entry(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        /*
         * First use as selector to avoid Poison, pick jtable entry
         */
        self.expr(meta)
            * (self.expr(meta) - constant_from!(SharedColumnTableSelector::ExecutionTable as u64))
            * (self.expr(meta) - constant_from!(SharedColumnTableSelector::MemoryTable as u64))
    }

    pub fn is_enable_jtable_entry_bit(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        /*
         * Normalize
         */
        self.is_enable_jtable_entry(meta) * constant!(F::from(6).invert().unwrap())
    }

    pub fn _is_enable_etable_entry(
        &self,
        meta: &mut VirtualCells<F>,
        rotation: Rotation,
    ) -> Expression<F> {
        /*
         * First use as selector to avoid Poison, pick etable entry
         */
        self.expr_rot(meta, rotation)
            * (self.expr_rot(meta, rotation)
                - constant_from!(SharedColumnTableSelector::ExecutionTable as u64))
            * (self.expr_rot(meta, rotation)
                - constant_from!(SharedColumnTableSelector::MemoryTable as u64))
    }

    pub fn is_enable_etable_entry_cur(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        self._is_enable_etable_entry(meta, Rotation::cur())
    }

    pub fn is_enable_etable_entry_next(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        self._is_enable_etable_entry(meta, Rotation(ETABLE_STEP_SIZE as i32))
    }

    pub fn is_enable_etable_entry_cur_bit(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        /*
         * Normalize
         */
        self.is_enable_etable_entry_cur(meta) * constant!(F::from(2).invert().unwrap())
    }
}

#[derive(Clone)]
pub struct TableBlockFirstLineSelector<F> {
    internal: Column<Fixed>,
    _mark: PhantomData<F>,
}

impl<F: FieldExt> TableBlockFirstLineSelector<F> {
    fn alloc(meta: &mut ConstraintSystem<F>) -> Self {
        TableBlockFirstLineSelector::<F> {
            internal: meta.fixed_column(),
            _mark: PhantomData,
        }
    }

    pub fn enable(&self, ctx: &mut Context<'_, F>) -> Result<(), Error> {
        ctx.region.as_ref().borrow_mut().assign_fixed(
            || "table selector",
            self.internal,
            (ctx.offset as i32) as usize,
            || Ok(F::one()),
        )?;

        Ok(())
    }

    pub fn expr(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        meta.query_fixed(self.internal, Rotation::cur())
    }
}

// pub fn assign(self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {}
pub struct SharedColumns<F: FieldExt> {
    advices: [Column<Advice>; SHARED_ADVICE_COLUMN],

    /// Indicate the different types of rows
    /// 1 for etable,
    /// 2 for mtable,
    /// 3 for frame table
    table_selector: TableSelectorColumn<F>,

    block_first_line_selector: TableBlockFirstLineSelector<F>,
}

impl<F: FieldExt> SharedColumns<F> {
    pub fn new(meta: &mut ConstraintSystem<F>) -> Self {
        SharedColumns {
            advices: [(); SHARED_ADVICE_COLUMN].map(|_| meta.advice_column()),
            table_selector: TableSelectorColumn::<F>::alloc(meta),
            block_first_line_selector: TableBlockFirstLineSelector::<F>::alloc(meta),
        }
    }

    pub fn advices_iter(&self) -> IntoIter<Column<Advice>, SHARED_ADVICE_COLUMN> {
        self.advices.into_iter()
    }

    pub fn get_table_selector(&self) -> TableSelectorColumn<F> {
        self.table_selector.clone()
    }

    pub fn get_block_first_line_selector(&self) -> TableBlockFirstLineSelector<F> {
        self.block_first_line_selector.clone()
    }
}
