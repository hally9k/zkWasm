use std::collections::BTreeMap;

use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{ConstraintSystem, Expression, VirtualCells},
};

use crate::foreign::foreign_trait::ForeignTableConfig;

pub struct ConstraintBuilder<'a, F: FieldExt> {
    meta: &'a mut ConstraintSystem<F>,
    constraints: Vec<(
        &'static str,
        Box<dyn FnOnce(&mut VirtualCells<F>) -> Vec<Expression<F>>>,
    )>,
    lookups: BTreeMap<
        &'static str,
        Vec<(
            &'static str,
            Box<dyn Fn(&mut VirtualCells<F>) -> Expression<F>>,
        )>,
    >,
}

impl<'a, F: FieldExt> ConstraintBuilder<'a, F> {
    pub fn new(meta: &'a mut ConstraintSystem<F>) -> Self {
        Self {
            meta,
            constraints: vec![],
            lookups: BTreeMap::new(),
        }
    }

    pub fn push(
        &mut self,
        name: &'static str,
        builder: Box<dyn FnOnce(&mut VirtualCells<F>) -> Vec<Expression<F>>>,
    ) {
        self.constraints.push((name, builder));
    }

    pub fn lookup(
        &mut self,
        foreign_table_id: &'static str,
        name: &'static str,
        builder: Box<dyn Fn(&mut VirtualCells<F>) -> Expression<F>>,
    ) {
        match self.lookups.get_mut(&foreign_table_id) {
            Some(lookups) => lookups.push((name, builder)),
            None => {
                self.lookups.insert(foreign_table_id, vec![(name, builder)]);
            }
        }
    }

    pub fn finalize(
        self,
        foreign_tables: &BTreeMap<&'static str, Box<dyn ForeignTableConfig<F>>>,
        enable: impl Fn(&mut VirtualCells<F>) -> Expression<F>,
    ) {
        for (name, builder) in self.constraints {
            self.meta.create_gate(&name, |meta| {
                builder(meta)
                    .into_iter()
                    .map(|constraint| constraint * enable(meta))
                    .collect::<Vec<_>>()
            });
        }

        for (id, lookups) in self.lookups {
            let config = foreign_tables.get(&id).unwrap();
            for (key, expr) in lookups {
                config.configure_in_table(self.meta, key, &|meta| expr(meta) * enable(meta));
            }
        }
    }
}
