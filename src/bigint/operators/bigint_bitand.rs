use crate::bigint::BigInt;

use std::ops::*;

impl BitAnd for BigInt {
    type Output = BigInt;

    fn bitand(self, b:BigInt) -> BigInt {
        return &self & &b;
    }
}

impl<'a> BitAnd<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn bitand(self, b:&'a BigInt) -> BigInt {
        return &self & b;
    }
}

impl<'a> BitAnd<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitand(self, b: BigInt) -> BigInt {
        return self & &b;
    }
}

impl<'a,'b> BitAnd<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn bitand(self, b: &'a BigInt) -> BigInt {

        if self.negative == true {
            return self.twos_complement() & b;
        }

        if b.negative == true {
            return self & b.twos_complement();
        }

        // Add each of the u64 for a&b until there aren't anymore
        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        for i in 0..std::cmp::min(self.data.len(), b.data.len()) {

            // Add the digit to the BigInt
            assert!(self.negative == false);
            assert!(b.negative == false);
            let temp = self.data[i] & b.data[i];

            result.data.push(temp);
        }

        // No need to look at tails, since it will always be 0 in a bitwise-and
        return result;
    }
}
