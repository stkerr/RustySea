use ::bigint::*;
use ::bigint::utilities::*;

use std::ops::*;

impl Shl<BigInt> for BigInt {
	type Output = BigInt;

	fn shl(self, b:BigInt) -> BigInt {
		if b.negative {
			let mut positive_b = b.clone();
			positive_b.negative = false;
			return self >> (positive_b);
		}

		let mut remaining:BigInt = b;
		let sixty_four:BigInt = create_bigint_from_string("40").unwrap();
		let mut new_data:Vec<u64> = vec![];

		println!("Remaining: {}", remaining.data[0]);
		println!("{}", remaining.compare(&sixty_four));
		while remaining.compare(&sixty_four) >= 0 {
			// Just append another zero
			new_data.push(0);
			println!("Shifting! {}", remaining);

			remaining = remaining - sixty_four.clone();
		}

		println!("Remaining: {}", remaining.data[0]);
		let mut out_shift_part:u64;
		let mut up_shift_part:u64;
		let mut previous_out_shift_part:u64 = 0;

		for i in 0..self.length {
			let mut mask:u64 = !0;

            let mut j = 0;
			while j < (64-remaining.data[0]) {
				mask = mask << 1 & (!1);
				j += 1;
			}

			out_shift_part = (self.data[i] & (mask)) >> remaining.data[0];
			up_shift_part = (self.data[i] & (!mask)) << remaining.data[0];
			println!("data: {:x}", self.data[i]);
			println!("mask: {:x}\n!mask: {:x}", mask, !mask);
			println!("previous_out_shift_part: {:x}", previous_out_shift_part);
			println!("out_shift_part: {:x}", out_shift_part);
			println!("up_shift_part: {:x}", up_shift_part);
			println!("i: {}", self.data[i]);

			new_data.push(previous_out_shift_part | up_shift_part);

			previous_out_shift_part = out_shift_part;
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
