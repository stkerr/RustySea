use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;

impl Rem for BigInt {
    type Output = BigInt;

    fn rem(self, b:BigInt) -> BigInt {
        return &self % &b;
    }
}

impl<'a> Rem<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, b:&'a BigInt) -> BigInt {
        return &self % b;
    }
}

impl<'a> Rem<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn rem(self, b: BigInt) -> BigInt {
        return self % &b;
    }
}

impl<'a,'b> Rem<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn rem(self, b: &'a BigInt) -> BigInt {
        if self.negative {
            let mut self_clone = self.clone();
            self_clone.negative = false;

            let mut temp = self_clone % b;
            temp = temp + b;

            return temp;
        } else {
            let one:BigInt = create_bigint_from_string("0x1").unwrap();
            let mut self_clone = self.clone();
            let mut prev_self_clone:BigInt = self.clone();
            let mut count:u8 = 0;
            while self_clone.compare(&b) > 0 {
                prev_self_clone = self_clone.clone();

                println!("Iterated. {} ({}) > {}", self_clone, prev_self_clone, b);
                self_clone = &self_clone >> &one;
                if count == 100 {
                    panic!("Done");
                }
                count = count + 1;
            }

            while prev_self_clone.compare(&b) >= 0 {
                prev_self_clone = &prev_self_clone - b;
            }
            println!("After clone shift, result {}", prev_self_clone);
            return prev_self_clone;
        }
    }
}
