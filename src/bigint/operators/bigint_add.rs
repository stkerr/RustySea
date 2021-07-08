use crate::bigint::BigInt;

use std;
use std::ops::*;

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, b:BigInt) -> BigInt {
        return &self + &b;
    }
}

impl<'a> Add<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, b: &'a BigInt) -> BigInt {
        return &self + b;
    }
}

impl<'a> Add<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn add(self, b: BigInt) -> BigInt {
        return self + &b;    
    }
}

impl<'a,'b> Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn add(self, b: &'a BigInt) -> BigInt {
        if self.negative && !b.negative {
            let self_copy:BigInt = BigInt { negative: false, data: self.data.clone()};
            let result:BigInt = b.subtract(&self_copy);
            return result;
        } else if !self.negative && b.negative {
            let b_copy:BigInt = BigInt { negative: false, data: b.data.clone()};
            let result:BigInt = self.subtract(&b_copy);
            return result;
        }

        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        if self.negative && b.negative {
            // Adding two negatives is the same as a normal add, just with the resulting sign
            // as negative
            result.negative = true;
        }

        // Add each of the u64 for a&b until there aren't anymore
        let mut carry:u64 = 0;
        for i in 0..std::cmp::min(self.data.len(), b.data.len()) {

            // Add the raw values
            let (interim, internal_carry) = crate::bigint::utilities::add_with_carry(self.data[i], b.data[i]);
            let temp_carry:u64= internal_carry;

            // Add the previous carry value
            let (interim, internal_carry) = crate::bigint::utilities::add_with_carry(interim, carry);
            carry = internal_carry + temp_carry;

            // Add the digit to the BigInt
            result.data.push(interim);
        }

        // Find the longer integer if it is there
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
                // println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.data.len() {
                    let (next, next_carry) = crate::bigint::utilities::add_with_carry(x.data[i], carry);
                    carry = next_carry;
                    result.data.push(next);
                }
            },
            None => {
                // Add the final carry if there is one
                if carry > 0 {
                    result.data.push(carry);
                }
            }
        }

        return result;
    }
}
