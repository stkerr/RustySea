use ::bigint::*;

// Returns a tuple of (low-order result, carry)
// where low-order result is a u64 and carry flag is a
// u64
pub fn add_with_carry(a: u64, b:u64) -> (u64, u64) {
    // Add the raw values
    let interim:Option<u64> = a.checked_add(b);

    match interim {
        Some(x) => {
            return (x, 0);
        },
        None => {
            let t1 = 0xFFFFFFFFFFFFFFFF-a;
            let t2 = 0xFFFFFFFFFFFFFFFF-b;
            let low_order:u64 = 0xFFFFFFFFFFFFFFFF - t1 - t2 - 1;
            
            return (low_order, 1);
        }
    };
}

// Returns a tuple of (low-order result, carry, negative)
// A carry with a negative result represents a borrow
pub fn signed_add_with_carry(a: u64, a_negative: bool, b:u64, b_negative: bool) -> (u64, u64, bool) {
    if a == 0 && b == 0 {
        return (0,0,false);
    }

    if a_negative && !b_negative {
        match a > b {
            true => return (u64::MAX-(a-b)+1, 1, true),
            false => return (b-a, 0, false)
        };
    } else if !a_negative && b_negative {
        match a > b {
            true => return (a-b, 0, false),
            false => return (u64::MAX-(b-a)+1, 1, true)
        };
    } else if !a_negative && !b_negative {
        let (x,y) = add_with_carry(a,b);
        return (x,y,false);
    } else {
        let (x,y) = add_with_carry(a,b);
        return (x,y,true);
    }
}

impl BigInt {

    // Returns -1 if self is smaller than b, 0 if self==b, 1 is self is greater than b.
    // This comparison is done ignoring sign. 
    fn compare_ignore_sign(&self, b: &BigInt) -> i8 {
        if self.data.len() > b.data.len() {
            return 1;
        } else if self.data.len() < b.data.len() {
            return -1;
        } else {
            for i in (0..self.data.len()).rev() {
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
    pub fn compare(&self, b: &BigInt) -> i8 {
        if self.negative && !b.negative {
            return -1;
        } else if !self.negative && b.negative {
            return 1;
        } else {
            if self.negative && b.negative {
                // We are comparing by absolute value here, but we
                // have two negative numbers, so flip the sign of the result this case.
                return -1*self.compare_ignore_sign(&b);
            } else {
                return self.compare_ignore_sign(&b);
            }
        }
    }

    pub fn subtract(&self, b: &BigInt) -> BigInt {
        return self - b;
    }

    pub fn add(&self, b: &BigInt) -> BigInt {
        return self + b;
    }
}

pub fn print_bigint(val: &BigInt) {
    println!("Stored values: {} ", val.data.len());
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

    if val.len() == 0 {
        return Err(Error::HexParsingError)
    }

    let mut the_data: Vec<u64> = Vec::new();

    let mut values:Vec<u8> = Vec::new();
    
    // Iterate over each character from right to left, since
    // we are taking the number in big-endian form.
    let mut is_negative:bool = false;

    let mut val_vec:Vec<char> = val.chars().collect::<Vec<char>>();

    // Process the negative symbol
    if val.starts_with("-") {
        is_negative = true;
        val_vec.remove(0);
    }

    // Process a leading 0x if there is one
    if val_vec.get(0) == Some(&'0') && val_vec.get(1) == Some(&'x') {
        val_vec.remove(0);
        val_vec.remove(0);
    } else {
        println!("Value was {}", val);
        println!("Got {:?} and {:?}", val_vec.get(0), val_vec.get(1));
        if cfg!(not(fuzzing)) {
            panic!("Decimal string mode not supported.");
        }
        else {
            return Err(Error::NotImplementedError);
        }
    }

    // Strip the leading zeros
    while val_vec.len() > 1 && val_vec.get(0) == Some(&'0') {
        val_vec.remove(0);
    }

    if val_vec.len() == 0 {
        return Err(Error::HexParsingError);
    }

    for c in val_vec.iter().rev() {

        let digit:Option<u32> = c.to_digit(16);
        
        match digit {
            e @ Some(0...0xFF) => values.insert(0, e.unwrap() as u8),
            _ => return Err(Error::HexParsingError)
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

    return Ok(BigInt { negative:is_negative, data: the_data});
}
