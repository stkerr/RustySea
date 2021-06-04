use ::bigint::BigInt;

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

        println!("0x{:x} 0x{:x} {} {}", self.data[0], b.data[0], self.negative, b.negative);
        if self.negative == true {
            return self.twos_complement() & b;
        }

        if b.negative == true {
            let one:BigInt  = ::bigint::utilities::create_bigint_from_string("0x1").unwrap();
            println!("Here");
            let b2:BigInt = !b + one;
            println!("b2 negative: {}", b2.negative);
            println!("B: 0x{:x}\nflipped b: 0x{:x}", b.data[0], b2.data[0]);
            return self & b.twos_complement();
        }

        // Add each of the u64 for a&b until there aren't anymore
        let mut result:BigInt = BigInt {negative: false, data: vec![] };
        for i in 0..std::cmp::min(self.data.len(), b.data.len()) {

            
            println!("0x{:x} 0x{:x} {} {}", self.data[0], b.data[0], self.negative, b.negative);
            // Add the digit to the BigInt
            assert!(self.negative == false);
            assert!(b.negative == false);
            let temp = self.data[i] & b.data[i];

            println!("bitand: 0x{:x} 0x{:x} => 0x{:x}", self.data[i], b.data[i], self.data[i]);
            result.data.push(temp);
        }

        // No need to look at tails, since it will always be 0 in a bitwise-and
        return result;
    }
}
