use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, b:BigInt) -> BigInt {
        return &self * &b;
    }
}

impl<'a> Mul<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, b:&'a BigInt) -> BigInt {
        return &self * b;
    }
} 
impl<'a> Mul<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn mul(self, b: BigInt) -> BigInt {
        return self * &b;
    }
}

impl<'a,'b> Mul<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn mul(self, b: &'a BigInt) -> BigInt {
        /*
         * From Wikipedia
        c = 0

        while b ≠ 0
            if (b and 1) ≠ 0
                c = c + a
            left shift a by 1
            right shift b by 1

        return c
        */
        let zero:BigInt  = create_bigint_from_string("0x0").unwrap();
        let one:BigInt = create_bigint_from_string("0x1").unwrap();
        let mut c:BigInt = zero.clone();
        let mut a_copy:BigInt = self.clone();
        let mut b_copy:BigInt = b.to_owned();
        while (b_copy.compare(&zero)) != 0 {
            if (&b_copy & &one).compare(&zero) != 0 {
                c = &c + &a_copy;
            }
            a_copy = &a_copy << &one;
            b_copy = &b_copy >> &one;
        }
        if (self.negative && !b.negative) || (!self.negative && b.negative) {
            if (self.compare_ignore_sign(zero) == 0) {
                // Don't return negative 0
                c.negative = false;
            }
            else {
                c.negative = true;    
            }
        } else {
            c.negative = false;
        }

        return c;
    }
}
