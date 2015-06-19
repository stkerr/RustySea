pub struct BigInt {
    length: usize,
    negative:bool,
    data: Vec<u64>
}

// Returns a tuple of (low-order result, carry)
// where low-order result is a u64 and carry flag is a
// u64
fn add_with_carry(a: u64, b:u64) -> (u64, u64) {
    // Add the raw values
    let interim:Option<u64> = a.checked_add(b);

    match interim {
        Some(x) => return (x, 0),
        None => {
            let low_order:u64 = match a < b {
                true => a-1,
                false => b-1
            };
            return (low_order, 1);
        }
    };
}

// Returns a tuple of (low-order result, carry, negative)
// A carry with a negative result represents a borrow
fn signed_add_with_carry(a: u64, a_negative: bool, b:u64, b_negative: bool) -> (u64, u64, bool) {
    if a_negative && !b_negative {
        match a > b {
            true => return (a-b, 0, true),
            false => return (b-a, 0, false)
        };
    } else if !a_negative && b_negative {
        match a > b {
            true => return (a-b, 0, false),
            false => return (b-a, 0, true)
        };
    } else if !a_negative && !b_negative {
        let (x,y) = add_with_carry(a,b);
        return (x,y,false);
    } else {
        let (x,y) = add_with_carry(a,b);
        return (x,y,true);
    }
}

#[derive(Debug)]
pub enum Error { NotImplementedError, HexParsingError }

impl BigInt {

    fn left_shift(&self, quantity: &BigInt) -> Result<BigInt, Error> {
        println!("Shifting:");
        print_bigint(&self);
        let mut result:BigInt = BigInt { length: 0, negative:false, data: Vec::new()};

        let mut bit_out = 0;
        for v in &self.data {
            println!("Shift Item {}", v);
            let new_item:u64 = v << 1 | bit_out;
            bit_out = v | 0x8000000000000000;
            bit_out = match bit_out {
                0 => 0,
                _ => 1
            };
            println!("New shift item {}", new_item);
            result.data.push(new_item);
            result.length = result.length + 1;
        }
        return Err(Error::NotImplementedError);
    }

    fn div(&self, b: &BigInt) -> Result<(BigInt, BigInt), Error> {
        return Err(Error::NotImplementedError);
    }

    fn mult(&self, b: &BigInt) -> Result<BigInt, Error> {
        // TODO: Make this implementation faster using Karatsuba
        // multiplication or almost any other algorithm.


        // We can write A*B=C as a series of doubles and adds
        // of A with itself.
        return Err(Error::NotImplementedError);
    }
    
    // Returns -1 if self is smaller than b, 0 if self==b, 1 is self is greater than b.
    // This comparison is done ignoring sign. 
    fn compare_ignore_sign(&self, b: &BigInt) -> i8 {
        if self.length > b.length {
            return 1;
        } else if self.length < b.length {
            return -1;
        } else {
            for i in 0..self.data.len() {
                if self.data[i] > b.data[i] {
                    return 1;
                } else if self.data[i] < b.data[i] {
                    return -1;
                }
            }
            return 0;
        }
    }

    // Returns -1 if self is smaller than b, 0 if self==b, 1 is self is greater than b.
    fn compare(&self, b: &BigInt) -> i8 {
        if self.negative && !b.negative {
            return -1;
        } else if !self.negative && b.negative {
            return 1;
        } else {
            if self.negative && b.negative {
                // We are comparing by absolute value here, but we
                // have two negative numbers, so flip the sign of the result this case.
                return !self.compare_ignore_sign(&b);
            } else {
                return self.compare_ignore_sign(&b);
            }
        }
    }

    fn subtract(&self, b: &BigInt) -> BigInt {
        if self.negative && b.negative {
            // Subtracting a negative is the same as adding the positive value
            let b_copy:BigInt = BigInt { length: b.length, negative: false, data: b.data.clone()};
            return self.add(&b_copy);
        } else if self.negative && !b.negative {
            // We are subtracing a positive, but we are already negative, so just add the absolute values
            // then call it negative
            let self_copy:BigInt = BigInt { length: self.length, negative: false, data: self.data.clone()};
            let b_copy:BigInt = BigInt { length: b.length, negative: false, data: b.data.clone()};
            let mut result:BigInt = self_copy.add(&b_copy);
            result.negative = true;
            return result;
        } else if !self.negative && b.negative {
            // Subtracting a negative is the same as adding the positive
            let b_copy:BigInt = BigInt { length: b.length, negative: false, data: b.data.clone()};
            return self.add(&b_copy);
        } else if !self.negative && !b.negative {
            // This is the case we actually need to handle below
        }
        
        let mut result:BigInt = BigInt {length: 0, negative: false, data: vec![] };
        let comparison:i8 = self.compare(b);

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
            let mut temp_is_negative:bool = false;
            for i in 0..std::cmp::min(self.length, b.length) {

                // Add the raw values
                let (interim, internal_borrow, temp_is_negative) = signed_add_with_carry(self.data[i], self.negative, b.data[i], true);
                let temp_borrow:u64= internal_borrow + borrow;
                println!("Interim: {} from {} and {}", interim, self.data[i], b.data[i]);
                // Add the previous borrow value
                let (interim, internal_borrow, temp_is_negative) = signed_add_with_carry(interim, temp_is_negative, temp_borrow, true);
                borrow = internal_borrow + temp_borrow;

                // Add the digit to the BigInt
                result.data.push(interim);
                result.length = result.length + 1;
            }

            // Find the longer integer if it is there
            let (longer, starting_index) = match self.length == b.length {
                true => (None, 0),
                false => match  self.length > b.length {
                    true => (Some(self), self.length - b.length),
                    false => (Some(b), b.length - self.length)
                }
            };

            // Add in the longer tail of the two values
            match longer {
                Some(x) => {
                    println!("Unequal sizes, parsing the longer.");
                    for i in starting_index..x.length {
                        let (next, next_borrow) = add_with_carry(x.data[i], borrow);
                        borrow = next_borrow;
                        result.data.push(next);
                        result.length = result.length + 1;
                    }
                },
                None => println!("Same length.")
            }
             
            // Subtract the final borrow if there is one
            if borrow > 0 {
                result.data.push(borrow);
                result.length = result.length + 1;
            }

            return result;

        } else {
            // We are subtracting the same value as ourself, so just return 0.
            return BigInt {length: 1, negative: false, data: vec![0]};
        }
    }

    fn add(&self, b: &BigInt) -> BigInt {
        if self.negative && !b.negative {
            let self_copy:BigInt = BigInt { length: self.length, negative: false, data: self.data.clone()};
            let result:BigInt = b.subtract(&self_copy);
            return result;
        } else if !self.negative && b.negative {
            let b_copy:BigInt = BigInt { length: b.length, negative: false, data: b.data.clone()};
            let result:BigInt = self.subtract(&b_copy);
            return result;
        }

        let mut result:BigInt = BigInt {length: 0, negative: false, data: vec![] };
        if self.negative && b.negative {
            // Adding two negatives is the same as a normal add, just with the resulting sign
            // as negative
            result.negative = true;
        }

        // Add each of the u64 for a&b until there aren't anymore
        let mut carry:u64 = 0;
        for i in 0..std::cmp::min(self.length, b.length) {

            // Add the raw values
            let (interim, internal_carry) = add_with_carry(self.data[i], b.data[i]);
            let temp_carry:u64= internal_carry + carry;

            // Add the previous carry value
            let (interim, internal_carry) = add_with_carry(interim, carry);
            carry = internal_carry + temp_carry;

            // Add the digit to the BigInt
            result.data.push(interim);
            result.length = result.length + 1;
        }

        // Find the longer integer if it is there
        let (longer, starting_index) = match self.length == b.length {
            true => (None, 0),
            false => match  self.length > b.length {
                true => (Some(self), self.length - b.length),
                false => (Some(b), b.length - self.length)
            }
        };

        // Add in the longer tail of the two values
        match longer {
            Some(x) => {
                println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.length {
                    let (next, next_carry) = add_with_carry(x.data[i], carry);
                    carry = next_carry;
                    result.data.push(next);
                    result.length = result.length + 1;
                }
                carry = 0; // no carry since we just added all the carry positions
            },
            None => println!("Same length.")
        }
         
        // Add the final carry if there is one
        if carry > 0 {
            result.data.push(carry);
            result.length = result.length + 1;
        }
        
        return result;
    }
}

fn main() {
    println!("Hello, world!");

/*
    // Create some integers. Make them overflow.
    let mut a = create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let mut b = create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let mut c = match a { 
        Ok(v) => v.add(&b),
        Err(e) => panic!(e)
    };

    let mut d = match c {
        Ok(v) => v.add(&b),
        Err(e) => panic!(e)
    };

    let (e, f, g) = signed_add_with_carry(0xFFFFFFFFFFFFFFFF, true, 0xFFFFFFFFFFFFFFFF, true);
    println!("signed a + b = {}. Carry/Borrow: {}. Negative: {}", e, f, g);
    // let (h, i, j) = signed_add_with_carry(0xFFFFFFFFFFFFFFFF, false, 0xFFFFFFFFFFFFFFFF, false);

    // -1 - 0xFFFFFFFFFFFFFFFF = -10000000000000000 (it did a borrow)
    let (h, i, j) = signed_add_with_carry(0x1, true, 0xFFFFFFFFFFFFFFFF, true);
    println!("-1 + (-0xFFFFFFFFFFFFFFFF) = {:x}. Carry/Borrow: {}. Negative: {}", h, i, j);

    // Print the results
    // print_bigint(&a);
    // print_bigint(&b);
    // print_bigint(&c);
    // print_bigint(&d);

    // Do 5+10=15
    let one_result = create_bigint_from_string("1");
    let one = match one_result { Ok(v) => v, Err(e) => panic!(e)};
    let five_result = create_bigint_from_string("5");
    let five = match five_result { Ok(v) => v, Err(e) => panic!(e)};
    let ten_result = create_bigint_from_string("10");
    let ten = match ten_result { Ok(v) => v, Err(e) => panic!(e)};
    let fifteen = five.add(&ten);
    print_bigint(&fifteen);

    println!("1 ?= 5 => {}", one.compare_ignore_sign(&five));
    println!("Larger ?= large => {}", c.compare_ignore_sign(&b));
    println!("large ?= larger => {}", b.compare_ignore_sign(&c));
    println!("larger ?= larger => {}", c.compare_ignore_sign(&c));

    println!("Larger ?= large => {}", c.compare(&b));
    println!("large ?= larger => {}", b.compare(&c));
    println!("larger ?= larger => {}", c.compare(&c));

    // Do some shifts
    let seven_result = create_bigint_from_string("7");
    let seven = match seven_result { Ok(v) => v, Err(e) => panic!(e)};
    let shifted:Result<BigInt, Error> = seven.left_shift(&one);
    match shifted {
        Ok(v) => print_bigint(&v),
        Err(e) => println!("Error {:?}", e)
    }
    
    // Try some adds with negative numbers
    println!("--- Addition & subtraction basic tests ----");
    println!("7 + (-1) ="); print_bigint(&create_bigint_from_string("7").add(&create_bigint_from_string("-1")));
    println!("(-7) + 1 ="); print_bigint(&create_bigint_from_string("-7").add(&create_bigint_from_string("1")));
    println!("(-7) + (-1) ="); print_bigint(&create_bigint_from_string("-7").add(&create_bigint_from_string("-1")));
    println!("7 - 1 ="); print_bigint(&create_bigint_from_string("7").subtract(&create_bigint_from_string("1")));
    println!("7 - (-1) ="); print_bigint(&create_bigint_from_string("7").subtract(&create_bigint_from_string("-1")));
    println!("(-7) - (-1) ="); print_bigint(&create_bigint_from_string("-7").subtract(&create_bigint_from_string("-1")));
    println!("(-7) - 1 ="); print_bigint(&create_bigint_from_string("-7").subtract(&create_bigint_from_string("1")));

    println!("Larger addition & subtraction tests ----");
    let a3 = a.add(&a).add(&a);
    let a2 = a.add(&a);
    println!("a2:");print_bigint(&a2);
    println!("a3:");print_bigint(&a3);
    println!("a3 + a2 = "); print_bigint(&a3.add(&a2));
*/
}

fn print_bigint(val: &BigInt) {
    println!("Stored values: {} ", val.length);
    println!("Negative: {}", val.negative);
    for i in &val.data {
        println!("Item: {:x}", i);
    }
}

fn convert_nibbles_to_u64(values: &[u8]) -> u64 {
    // We need to turn this buffer into a u64 now
    let mut temp:u64 = 0;
    for &i in values {
        temp = temp << 4 | i as u64;
    }
    return temp;
}


pub fn create_bigint_from_string(val: &str) -> Result<BigInt, Error> {

    if(val.len() == 0) {
        return Err(Error::HexParsingError)
    }

    println!("Len(str): {}", val.len());
    let mut the_data: Vec<u64> = Vec::new();

    let mut values:Vec<u8> = Vec::new();
    
    // Iterate over each character from right to left, since
    // we are taking the number in big-endian form.
    let mut is_negative:bool = false;

    // We should remove leading zeros before starting to save data
    let mut should_remove_zeros = true;

    for c in val.chars().rev() {

        if c == '-' {
            is_negative = true;
            continue;
        }

        if c != '0' && should_remove_zeros {
            // We got a non-zero number, so no longer remove a
            //leading zero.
            should_remove_zeros = false;
        } else if c == '0' && should_remove_zeros {
            continue;
        }

        let digit:Option<u32> = c.to_digit(16);
        
        match digit {
            e @ Some(0...0xFF) => values.insert(0, e.unwrap() as u8),
            _ => println!("Invalid character {}", c)
        };
    
        // If we have enough nibbles for a 64-bit integer, make one
        if values.len() == 64/4 {
            let temp:u64 = convert_nibbles_to_u64(&values);
            the_data.push(temp);

            // Clear all the entries out
            values.clear();
        }
    }

    // Parse out any remaining data
    if values.len() > 0 {
        let temp:u64 = convert_nibbles_to_u64(&values);
        the_data.push(temp);
    }

    return Ok(BigInt { length:the_data.len(), negative:is_negative, data: the_data});
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parsing() {
        let mut a:Result<BigInt,Error>;
        a = create_bigint_from_string("3");
        match a {
            Ok(v) => {
                assert!(v.length == 1);
                assert!(v.data[0] == 3);
                assert!(v.negative == false);
            }
            Err(e) => panic!(e)
        };

        a = create_bigint_from_string("-3");
        match a {
            Ok(v) => {
                assert!(v.length == 1);
                assert!(v.data[0] == 3);
                assert!(v.negative == true);
            }
            Err(e) => panic!(e)
        }
    }

    #[test]
    fn test_string_parsing_empty_string() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == false);    
    }

    #[test]
    fn test_string_parsing_negativeonly_string() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("-");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == false);    
    }

    #[test]
    fn test_string_parsing_invalid_string() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("Hello world!");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
    }

    #[test]
    fn test_string_parsing_zero() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("0");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == false);
    }

    #[test]
    fn test_string_parsing_negative_zero() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("-0");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == false);
    }

    #[test]
    fn test_string_parsing_leadingzeros() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("0000");
        let mut a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == false);

        a_result = create_bigint_from_string("-00003");
        a = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 3);
        assert!(a.negative == true);

        a_result = create_bigint_from_string("00003");
        a = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 3);
        assert!(a.negative == false);
    }

    #[test]
    fn test_string_parsing_negativezero_length_two() {
        let a_result:Result<BigInt,Error> = create_bigint_from_string("00000000000000000000000000000000000000000000000000000000000000");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 1);
        assert!(a.data[0] == 0);
        assert!(a.negative == true);
    }

    #[test]
    fn test_string_parsing_length_two() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("3FFFFFFFFFFFFFFFF");
        let mut a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 2);
        assert!(a.data[0] == 0xFFFFFFFFFFFFFFFF);
        assert!(a.data[1] == 3);
        assert!(a.negative == false);

        a_result = create_bigint_from_string("-3FFFFFFFFFFFFFFFF");
        a = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        assert!(a.length == 2);
        assert!(a.data[0] == 0xFFFFFFFFFFFFFFFF);
        assert!(a.data[1] == 3);
        assert!(a.negative == true);

    }

    #[test]
    fn test_basic_add_positive_positive_nocarry() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("7");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == false);
        assert!(c.data[0] == 14);
    }

    #[test]
    fn test_basic_add_positive_negative_nocarry_positiveresult() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-3");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == false);
        assert!(c.data[0] == 4);
    }

    #[test]
    fn test_basic_add_positive_negative_nocarry_negativeresult() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-8");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == true);
        assert!(c.data[0] == 1);
    }

    #[test]
    fn test_basic_add_negative_positive_nocarry_positiveresult() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-7");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("8");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == false);
        assert!(c.data[0] == 1);
    }

    #[test]
    fn test_basic_add_negative_positive_nocarry_negativeresult() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-7");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("3");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == true);
        assert!(c.data[0] == 4);
    }    

    #[test]
    fn test_basic_add_negative_negative_nocarry() {
        let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-3");
        let a:BigInt = match a_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };

        let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-3");
        let b:BigInt = match b_result {
            Ok(v) => v,
            Err(e) => panic!(e)
        };
        let c:BigInt = a.add(&b);
        assert!(c.length == 1);
        assert!(c.negative == true);
        assert!(c.data[0] == 6);
    }
}
