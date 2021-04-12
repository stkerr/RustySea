use ::bigint::BigInt;

use std::ops::*;

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, b: BigInt) -> BigInt {
        return &self - &b;
    }
}

impl<'a> Sub<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, b:&'a BigInt) -> BigInt {
        return &self - b;
    }
}

impl<'a> Sub<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn sub(self, b: BigInt) -> BigInt {
        return self - &b;
    }
}

impl<'a,'b> Sub<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn sub(self, b: &'a BigInt) -> BigInt {
        if self.negative && b.negative {
            // Subtracting a negative is the same as adding the positive value
            let b_copy:BigInt = BigInt { negative: false, data: b.data.clone()};
            return self + b_copy;
        } else if self.negative && !b.negative {
            // We are subtracing a positive, but we are already negative, so just add the absolute values
            // then call it negative
            let self_copy:BigInt = BigInt { negative: false, data: self.data.clone()};
            let b_copy:BigInt = BigInt { negative: false, data: b.data.clone()};
            let mut result:BigInt = self_copy + b_copy;
            result.negative = true;
            return result;
        } else if !self.negative && b.negative {
            // Subtracting a negative is the same as adding the positive
            let b_copy:BigInt = BigInt { negative: false, data: b.data.clone()};
            return self + b_copy;
        } else if !self.negative && !b.negative {
            // This is the case we actually need to handle below
        }
        
        let mut result:BigInt = BigInt { negative: false, data: vec![] };
        let comparison:i8 = self.compare(&b);

        if comparison < 0 {
            // Rather than do the subtraction here, do -1*(b-self). Since b>self, we
            // know we will have a negative result.
            result = b.subtract(&self);
            result.negative = true;
            return result;
        } else if comparison > 0 {
            // We are subtracting something smaller than ourself, so we will not be negative.
            // Do the actual subtraction operation here.

            // Add each of the u64 for a&b until there aren't anymore
            let mut borrow:u64 = 0;
            println!("Self.negative == {}", self.negative);
            for i in 0..b.data.len() {
                println!("self.data: {:x}", self.data[i]);
                println!("b.data: {:x}", b.data[i]);
                println!("borrow: {}", borrow);
                // Add the raw values
                let (interim, internal_borrow, temp_is_negative) = ::bigint::utilities::signed_add_with_carry(self.data[i], self.negative, b.data[i], true);
                println!("interim: {:x}\ninternal_borrow: {:x}\ntemp_is_negative: {}\n", interim, internal_borrow, temp_is_negative);
                let temp_borrow:u64= borrow;
                borrow = internal_borrow;
                // Subtract the previous borrow value
                let (interim, internal_borrow, _) = ::bigint::utilities::signed_add_with_carry(interim, false, temp_borrow, true);
                //borrow = internal_borrow + temp_borrow;

                // This operation is subtracting a borrow value only so shouldnt cause its own borrow.
                assert!(internal_borrow==0);
                println!("interim2: {:x}\ninternal_borrow2: {:x}", interim, internal_borrow);
                // Add the digit to the BigInt
                result.data.push(interim);
            }

            // Find the longer integer if it is there
            println!("Lengths: {} ?== {}\n", self.data.len(), b.data.len());
            let (longer, starting_index) = match self.data.len() == b.data.len() {
                true => (None, 0),
                false => match  self.data.len() > b.data.len() {
                    true => (Some(self), self.data.len() - (self.data.len() - b.data.len())),
                    false => (Some(b), b.data.len() - (b.data.len() - self.data.len()))
                }
            };

            // Add in the longer tail of the two values
            match longer {
                Some(x) => {
                    println!("Unequal sizes, parsing the longer.");
                    println!("{} - {}", self, b);
                    println!("starting_index:{}\n", starting_index);

                    for i in starting_index..(x.data.len()) {
                        println!("Doing {:x} - {:x}\n", x.data[i], borrow);
                        let (next, next_borrow, _) = ::bigint::utilities::signed_add_with_carry(x.data[i], false, borrow, true);
                        borrow = next_borrow;
                        result.data.push(next);
                        println!("Pushed {:x}\nnext_borrow: {}", next, next_borrow);
                    }
                },
                None => {}
            }
             
            // Subtract the final borrow if there is one
            if borrow > 0 {
                println!("Final borrow is {}\n", borrow);
                result.data.push(borrow);
            }

            while result.data.len() > 0 && result.data[result.data.len()-1] == 0 {
                // Clean-up leading 0 bytes
                result.data.pop();
            }

            return result;

        } else {
            // We are subtracting the same value as ourself, so just return 0.
            return BigInt {negative: false, data: vec![0]};
        }
    }
}
