use crate::bigint::BigInt;

use std::ops::*;

impl Not for &BigInt {
    type Output = BigInt;

    fn not(self) -> Self::Output {
        return !(self.clone());
    }
}

impl Not for BigInt {

    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result:BigInt = BigInt {negative: !self.negative, data: vec![] };

        println!("Bitnot start: {}", self);
        // Convert to 2's complement
        for i in 0..self.data.len() {    

            let temp = !self.data[i];

            println!("bitnot: 0x{:x} 0x{:x}", self.data[i], temp);

			result.data.push(temp);
        }

        // Add one
        let one:BigInt = crate::bigint::utilities::create_bigint_from_string("0x1").unwrap();
        result = result + one;

        // Switch from 2's complement back to decimal
        let mut result2:BigInt = BigInt {negative: !self.negative, data: vec![] };
        for i in 0..self.data.len() {    

            let temp = !result.data[i];

            println!("bitnot2: 0x{:x} 0x{:x}", result.data[i], temp);

			result2.data.push(temp);
        }

        return result2;
    }
}
