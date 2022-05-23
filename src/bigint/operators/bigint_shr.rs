use crate::bigint::BigInt;
use crate::bigint::utilities::*;

use std::ops::*;

impl Shr<BigInt> for BigInt {
    type Output = BigInt;

    fn shr(self, b:BigInt) -> BigInt {
        return &self >> &b;
    }
}

impl<'a> Shr<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn shr(self, b: &'a BigInt) -> BigInt {
        return &self >> b;
    }
}

impl<'a> Shr<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn shr(self, b: BigInt) -> BigInt {
        return self >> &b;    
    }
}

impl<'a,'b> Shr<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn shr(self, b:&'a BigInt) -> BigInt {

        // Negative bit-shifts are unsupported in ISO 9899 (C-language spec)
        assert!(b.negative == false);

        let mut remaining:BigInt = b.clone();
        let sixty_four:BigInt = create_bigint_from_string("0x40").unwrap();
        let mut new_data:Vec<u64> = vec![];
        let mut start_index = self.data.len();

        while remaining.compare(&sixty_four) >= 0 && start_index > 0 {
            // Just remove the lowest entry

            remaining = remaining - &sixty_four;
            start_index -= 1;
        }

        let mut out_shift_part:u64;
        let mut down_shift_part:u64;
        let mut previous_out_shift_part:u64 = 0;

        match start_index == 0 {
            true => {
            },
            false => {
                match remaining.data[0] == 0 {
                    false => {
                        for i in self.data.iter().rev().take(start_index) {
                            let mut mask:u64 = !0;

                            let mut j = 0;
                            while j < (remaining.data[0]) {
                                mask = mask << 1 & (!1);
                                j += 1;
                            }
                            out_shift_part = (i & (!mask)) << (64-remaining.data[0]); 
                            down_shift_part = (i & (mask)) >> (remaining.data[0]);

                            new_data.insert(0, previous_out_shift_part | down_shift_part);
                            previous_out_shift_part = out_shift_part;
                        }
                    },
                    true => {
                        for i in 0..start_index {
                            new_data.insert(0, self.data[self.data.len()-i-1]);

                        }

                    }
                }
            }
        }

        while new_data.len() > 0 && new_data[new_data.len()-1] == 0 {
            new_data.pop();
        }

        if new_data.len() == 0 {
            new_data.push(0);
        } 

        let result:BigInt = BigInt {negative: self.negative, data: new_data.clone() };
        return result;

    }
}

impl Shr<u8> for BigInt {
    type Output = BigInt;

    fn shr(self, b:u8) -> BigInt {
        panic!("Right shift by {} (by u8) not implemented.", b)
    }
}

impl Shr<u16> for BigInt {
    type Output = BigInt;

    fn shr(self, b:u16) -> BigInt {
        panic!("Right shift {} (by u16) not implemented.", b)
    }
}

impl Shr<u32> for BigInt {
    type Output = BigInt;

    fn shr(self, b:u32) -> BigInt {
        panic!("Right shift by {} (by u32) not implemented.", b)
    }
}
