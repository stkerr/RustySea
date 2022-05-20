use crate::bigint::BigInt;

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
        let mut result:BigInt = BigInt {negative: false, data: vec![] };

        let mut flip:bool = false;
        let mut a:BigInt = self.clone();
        let mut b_copy = b.clone();

        if self.negative == true {
            flip = true;
            a = self.twos_complement();
        }

        if b.negative == true {
            flip = true;
            b_copy = b_copy.twos_complement();
        }

        if self.data.len() < b.data.len() {
            for _i in 0..b.data.len()-a.data.len() {
                println!("Adding block a.");
                if self.negative {
                    // if negative, add a two's complement -1 block
                    a.data.push(0xffffffffffffffff);
                } else {
                    a.data.push(0);
                }
            }
        } else if a.data.len() > b.data.len() {
            for _i in 0..a.data.len()-b_copy.data.len() {
                
                if b.negative {
                    println!("Adding block b.");
                    // if negative, add a two's complement -1 block
                    b_copy.data.push(0xffffffffffffffff);
                } else {
                    b_copy.data.push(0);
                }
            }
        }

        if flip {
            let mut temp:BigInt = a ^ b_copy;

            if self.negative != b.negative {
                temp.negative = true;
                temp = temp.twos_complement();
                temp.negative = true;
            }
            

            // We might have a leading 0 block, so drop it if so
            if temp.data[temp.data.len()-1] == 0 {
                temp.data.pop();
            }

            return temp;
        }

        // Add each of the u64 for a&b until there aren't anymore
        for i in 0..std::cmp::min(a.data.len(), b_copy.data.len()) {

            // Add the digit to the BigInt
            assert!(a.negative == false);
            assert!(b_copy.negative == false);
            let temp = a.data[i] ^ b_copy.data[i];

            result.data.push(temp);
        }

        // We might have a leading 0 block, so drop it if so
        if result.data[result.data.len()-1] == 0 {
            result.data.pop();
        }

        // No need to look at tails, since it will always be 0 in a bitwise-and
        result.negative = self.negative ^ b.negative;
        return result;
    }
}
