use ::bigint::BigInt;

use std::ops::*;

impl Neg for BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        let mut returnable = self.clone();
        returnable.negative = !returnable.negative;
        return returnable;
    }
}
