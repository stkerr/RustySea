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
        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        for i in 0..std::cmp::min(self.data.len(), b.data.len()) {

            // Add the digit to the BigInt
            result.data.push(self.data[i] & b.data[i]);
        }

        let (longer, starting_index) = match self.data.len() == b.data.len() {
            true => (None, 0),
            false => match self.data.len() > b.data.len() {
                true => (Some(self), b.data.len()),
                false => (Some(b), self.data.len())
            }
        };

        // Add in the longer tail of the two values
        match longer {
            Some(x) => {
                println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.data.len() {
                    result.data.push(x.data[i]);
                }
            },
            None => {}
        }

        return result;
    }
}
