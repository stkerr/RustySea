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
        
        let a:BigInt;
        let b_copy:BigInt;
        let one:BigInt = crate::bigint::utilities::create_bigint_from_string("0x1").unwrap();
        let mut flip:bool = false;

        if self.negative {
            a = (self).twos_complement();
            flip = true;
        } else {
            a = self.clone();
        }
        if b.negative {
            b_copy = (b).twos_complement();
            flip = true;
        } else {
            b_copy = b.clone();
        }

        // Add each of the u64 for a&b until there aren't anymore
        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        for i in 0..std::cmp::min(a.data.len(), b_copy.data.len()) {

            // Add the digit to the BigInt
            result.data.push(a.data[i] | b_copy.data[i]);
        }

        let (longer, starting_index) = match a.data.len() == b.data.len() {
            true => (None, 0),
            false => match a.data.len() > b_copy.data.len() {
                true => (Some(a), b_copy.data.len()),
                false => (Some(b_copy), self.data.len())
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
        
        if flip {
            result.negative=true;
            result = (result).twos_complement();
            result.negative=true;
        }

        return result;
    }
}
