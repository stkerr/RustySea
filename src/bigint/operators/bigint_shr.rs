use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;

impl Shr<BigInt> for BigInt {
	type Output = BigInt;

	fn shr(self, b:BigInt) -> BigInt {
        if b.negative {
            let mut positive_b = b.clone();
            positive_b.negative = false;
            return self << positive_b;
        }

		let mut remaining:BigInt = b;
		let sixty_four:BigInt = create_bigint_from_string("40").unwrap();
		let mut new_data:Vec<u64> = vec![];
		let mut start_index = self.length;

		while remaining.compare(&sixty_four) >= 0 {
			// Just remove the lowest entry
			println!("Shifting! {}", remaining);

			remaining = remaining - sixty_four.clone();
			start_index -= 1;
		}

		let mut out_shift_part:u64;
		let mut down_shift_part:u64;
		let mut previous_out_shift_part:u64 = 0;

        match start_index == 0 {
            true => {
                new_data.push(0);
            },
            false => {
                for i in self.data.iter().rev().take(start_index) {
                    let mut mask:u64 = !0;

                    let mut j = 0;
                    while j < (remaining.data[0]) {
                        mask = mask << 1 & (!1);
                        j += 1;
                    }

                    out_shift_part = i & (!mask) << remaining.data[0]; 
                    down_shift_part = (i & (mask)) >> remaining.data[0];
                    
                    new_data.insert(0, previous_out_shift_part | down_shift_part);

                    previous_out_shift_part = out_shift_part;
                }
            }
        }

		let result:BigInt = BigInt {negative: self.negative, data: new_data.clone(), length: new_data.len() };
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
