use ::bigint::BigInt;

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
		// Add each of the u64 for a&b until there aren't anymore
		let mut result:BigInt = BigInt {length: 0, negative: false, data: vec![] };
        for i in 0..std::cmp::min(self.length, b.length) {

            // Add the digit to the BigInt
            println!("self.data[i] & b.data[i]: {}", self.data[i] & b.data[i]);
            result.data.push(self.data[i] | b.data[i]);
            result.length = result.length + 1;
        }

        let difference_result;
        match self.length > b.length {
            true => { difference_result = self.length - b.length; },
            false => { difference_result = b.length - self.length; },
        }
        let (longer, starting_index) = match self.length == b.length {
            true => (None, 0),
            false => match self.length > b.length {
                true => (Some(self), difference_result),
                false => (Some(b), difference_result)
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
