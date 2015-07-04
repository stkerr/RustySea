use ::bigint::BigInt;

use std;
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
        // Add each of the u64 for a&b until there aren't anymore
        let mut result:BigInt = BigInt {length: 0, negative: false, data: vec![] };
        for i in 0..std::cmp::min(self.length, b.length) {

            // Add the digit to the BigInt
            println!("self.data[i] & b.data[i]: {}", self.data[i] & b.data[i]);
            result.data.push(self.data[i] & b.data[i]);
            result.length = result.length + 1;
        }

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
                println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.length {
                    result.data.push(x.data[i]);
                    result.length = result.length + 1;
                }
            },
            None => {}
        }

        return result;
    }
}
