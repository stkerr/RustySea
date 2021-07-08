use crate::bigint::BigInt;

use std::ops::*;

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, b:BigInt) -> BigInt {
        return &self / &b;
    }
}

impl<'a> Div<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, b:&'a BigInt) -> BigInt {
        return &self / b;
    }
}

impl<'a> Div<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn div(self, b: BigInt) -> BigInt {
        return self / &b;
    }
}

impl<'a,'b> Div<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn div(self, b: &'a BigInt) -> BigInt {
        panic!("Div not implemented for {} / {}.", self, b);
    }
}
