struct BigInt {
    length: usize,
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

#[derive(Debug)]
enum Error { NotImplementedError }

impl BigInt {

    fn left_shift(&self, quantity: &BigInt) -> Result<BigInt, Error> {
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

    fn add(&self, b: &BigInt) -> BigInt {
        let mut result:BigInt = BigInt {length:0, data: vec![] };

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

        // Add in the longer of the two values
        let (longer, starting_index) = match self.length == b.length {
            true => (None, 0),
            false => match  self.length > b.length {
                true => (Some(self), self.length - b.length),
                false => (Some(b), b.length - self.length)
            }
        };

        // Find the longer integer if it is there
        match longer {
            Some(x) => {
                println!("Unequal sizes, parsing the longer.");
                for i in starting_index..x.length {
                    let (next, next_carry) = add_with_carry(x.data[i], carry);
                    carry = next_carry;
                    result.data.push(next);
                    result.length = result.length + 1;
                }
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

    // Create some integers. Make them overflow.
    let a = create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let b = create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let c = a.add(&b);
    let d = c.add(&b);

    // Print the results
    print_bigint(&a);
    print_bigint(&b);
    print_bigint(&c);
    print_bigint(&d);

    // Do 5+10=15
    let one = create_bigint_from_string("1");
    let five = create_bigint_from_string("5");
    let ten = create_bigint_from_string("10");
    let fifteen = five.add(&ten);
    print_bigint(&fifteen);

    // Do some shifts
    let seven = create_bigint_from_string("7");
    let shifted:Result<BigInt, Error> = seven.left_shift(&one);
    match shifted {
        Ok(v) => print_bigint(&v),
        Err(e) => println!("Error {:?}", e)
    }
    
}

fn print_bigint(val: &BigInt) {
    println!("Stored values: {} ", val.length);
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


fn create_bigint_from_string(val: &str) -> BigInt {
    let mut the_data: Vec<u64> = Vec::new();

    let mut values:Vec<u8> = Vec::new();
    
    // Iterate over each character from right to left, since
    // we are taking the number in big-endian form.

    for c in val.chars().rev() {
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

    return BigInt { length:the_data.len(), data: the_data} ;
}
