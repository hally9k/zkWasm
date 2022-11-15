use halo2_proofs::{arithmetic::FieldExt, plonk::Expression};
use num_bigint::BigUint;

use crate::{constant_from, constant_from_bn, expr::bn_to_field};

pub mod encode;

pub trait FromBn {
    fn zero() -> Self;
    fn from_bn(bn: &BigUint) -> Self;
}

impl<F: FieldExt> FromBn for Expression<F> {
    fn from_bn(bn: &BigUint) -> Self {
        constant_from_bn!(bn)
    }

    fn zero() -> Self {
        constant_from!(0)
    }
}

impl FromBn for BigUint {
    fn from_bn(bn: &BigUint) -> Self {
        bn.clone()
    }

    fn zero() -> Self {
        BigUint::from(0u64)
    }
}
