use std::ops::{BitAnd, Mul, Shr};

use num::{
    traits::{NumAssignRef, NumRef},
    BigUint,
};

pub trait UInt:
    NumRef
    + NumAssignRef
    + Shr<u64, Output = Self>
    + BitAnd<Output = Self>
    + Clone
    + for<'a> Mul<&'a Self>
//where
//    for<'b> &'b Self: Mul,
{
    fn bits(&self) -> u64;

    fn mul(&self, rhs: &Self) -> Self;
}

/*
impl<T: UInt> Mul for &T {

}
*/

impl UInt for u64 {
    fn bits(&self) -> u64 {
        if self == &0 {
            return 0;
        }
        self.ilog2() as u64 + 1
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl UInt for BigUint {
    fn bits(&self) -> u64 {
        self.bits()
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}
