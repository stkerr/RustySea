use ::bigint::BigInt;

use std;
use std::ops::*;

impl BitXor for BigInt {
    type Output = BigInt;

    fn bitxor(self, b:BigInt) -> BigInt {
        return &self ^ &b;
    }
}

impl<'a> BitXor<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn bitxor(self, b:&'a BigInt) -> BigInt {
        return &self ^ b;
    }
}

impl<'a> BitXor<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitxor(self, b: BigInt) -> BigInt {
        return self ^ &b;
    }
}

impl<'a,'b> BitXor<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    /// Note that this sets the sign of the result equal to the sign of the first
    /// parameter. TODO: Investigate if a different behavior should be used.
    fn bitxor(self, b: &'a BigInt) -> BigInt {
        let mut result:BigInt = BigInt {length: 0, negative: false, data: vec![] };
        result.negative = self.negative;

        for i in 0..std::cmp::min(self.length, b.length) {
            // Add the digit to the BigInt
            result.data.push(self.data[i] ^ b.data[i]);
            result.length = result.length + 1;
        }

        // Find the longer integer if it is there
        let (longer, starting_index) = match self.length == b.length {
            true => (None, 0),
            false => match self.length > b.length {
                true => (Some(self), b.length),
                false => (Some(b), self.length)
            }
        };

        // Add in the longer tail of the two values
        match longer {
            Some(x) => {
                // println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.length {
                    result.data.push(x.data[i]);
                    result.length = result.length + 1;
                }
            },
            None => {
            }
        }
        
        return result;
    }
}
