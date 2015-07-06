use ::bigint::*;
use ::bigint::utilities::*;

use std::ops::*;

impl Shl<BigInt> for BigInt {
    type Output = BigInt;

    fn shl(self, b:BigInt) -> BigInt {
        return &self << &b;
    }
}

impl<'a> Shl<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn shl(self, b: &'a BigInt) -> BigInt {
        return &self + b;
    }
}

impl<'a> Shl<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn shl(self, b: BigInt) -> BigInt {
        return self + &b;    
    }
}

impl<'a,'b> Shl<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn shl(self, b:&'a BigInt) -> BigInt {
        if b.negative {
            let mut positive_b = b.clone();
            positive_b.negative = false;
            return self >> positive_b;
        }

        let mut remaining:BigInt = b.clone();
        let sixty_four:BigInt = create_bigint_from_string("0x40").unwrap();
        let mut new_data:Vec<u64> = vec![];

        while remaining.compare(&sixty_four) >= 0 {
            // Just append another zero
            new_data.push(0);

            remaining = remaining - sixty_four.clone();
        }

        println!("Remaining: {}", remaining.data[0]);
        
        let mut out_shift_part:u64 = 0;
        let mut up_shift_part:u64 = 0;
        let mut previous_out_shift_part:u64 = 0;

        match remaining.data[0] == 0 {
            false => {

                for i in 0..self.length {
                    let mut mask:u64 = !0;

                    let mut j = 0;
                    while j < (remaining.data[0]) {
                        mask = mask >> 1;
                        j += 1;
                    }

                    println!("Mask {:x}", mask);
                    println!("Data: {:x}", self.data[i]);

                    out_shift_part = (self.data[i] & (!mask)) >> (64 - remaining.data[0]);
                    println!("Out shift {:x}", out_shift_part);

                    up_shift_part = (self.data[i] & (mask)) << (remaining.data[0]);
                    println!("Up shift {:x}", up_shift_part);

                    println!("data: {:x}", self.data[i]);
                    println!("mask: {:x}\n!mask: {:x}", mask, !mask);
                    println!("previous_out_shift_part: {:x}", previous_out_shift_part);
                    println!("out_shift_part: {:x}", out_shift_part);
                    println!("up_shift_part: {:x}", up_shift_part);
                    println!("i: {:x}", self.data[i]);

                    println!("Inserting {:x}", previous_out_shift_part | up_shift_part);
                    new_data.push(previous_out_shift_part | up_shift_part);

                    previous_out_shift_part = out_shift_part;
                }
            },
            true => {
                // Just append the old bytes directly
                for i in 0..self.length {
                    new_data.push(self.data[i]);
                }
            }
        }

        if out_shift_part != 0 {
            new_data.push(out_shift_part);
        }

        if new_data.len() == 0 {
            new_data.push(0);
        }
        for d in 0..new_data.len() {
            println!("new_data[{:x}]: {:x}", d, new_data[d]);   
        }
        
        let result:BigInt = BigInt {negative: self.negative, data: new_data.clone(), length: new_data.len() };
        println!("Result: {}", result);
        return result;
    }
}

impl Shl<u8> for BigInt {
    type Output = BigInt;

    fn shl(self, b:u8) -> BigInt {
        panic!("Left shift by {} (by u8) not implemented.", b)
    }
}

impl Shl<u16> for BigInt {
    type Output = BigInt;

    fn shl(self, b:u16) -> BigInt {
        panic!("Left shift by {} (by u16) not implemented.", b)
    }
}

impl Shl<u32> for BigInt {
    type Output = BigInt;

    fn shl(self, b:u32) -> BigInt {
        panic!("Left shift by {} (by u32) not implemented.", b)
    }
}
