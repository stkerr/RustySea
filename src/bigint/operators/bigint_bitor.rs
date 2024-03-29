use crate::bigint::BigInt;

use std;
use std::ops::*;

impl BitOr for BigInt {
    type Output = BigInt;

    fn bitor(self, b:BigInt) -> BigInt {
        return &self | &b;
    }
}

impl<'a> BitOr<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn bitor(self, b:&'a BigInt) -> BigInt {
        return &self | b;
    }
}

impl<'a> BitOr<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitor(self, b: BigInt) -> BigInt {
        return self | &b;
    }
}

impl<'a,'b> BitOr<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn bitor(self, b: &'a BigInt) -> BigInt {
        
        let mut a:BigInt;
        let mut b_copy:BigInt;
        let mut flip:bool = false;
        
        a = self.clone();
        b_copy = b.clone();

        if self.data.len() < b.data.len() {
            for _i in 0..b.data.len()-a.data.len() {
                a.data.push(0);
            }
        } else if a.data.len() > b.data.len() {
            for _i in 0..a.data.len()-b_copy.data.len() {
                b_copy.data.push(0);
            }
        }

        if self.negative {
            a = a.twos_complement();
            flip = true;
        }

        if b.negative {
            b_copy = b_copy.twos_complement();
            flip = true;
        }

        if flip {
            // if we need to do a sign flip, do that and return the result of the flipped numbers
            let mut temp:BigInt = a | b_copy;
            temp.negative=true; // ensure that 2's complement actually sees the value as negative
            temp = temp.twos_complement();
            temp.negative=true;

            // We might have a leading 0 block, so drop it if so
            if temp.data[temp.data.len()-1] == 0 {
                temp.data.pop();
            }

            return temp;
        }

        assert!(self.negative == false);
        assert!(b.negative == false);

        // Add each of the u64 for a&b until there aren't anymore
        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        for i in 0..std::cmp::min(a.data.len(), b_copy.data.len()) {

            // Add the digit to the BigInt
            result.data.push(a.data[i] | b_copy.data[i]);
        }

        return result;
    }
}
