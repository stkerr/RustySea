use ::bigint::BigInt;

use std;
use std::ops::*;

impl Shl<BigInt> for BigInt {
	type Output = BigInt;

	fn shl(self, b:BigInt) -> BigInt {
		panic!("Left shift (by BigInt) not implemented.")
	}
}

impl Shl<u8> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u8) -> BigInt {
		panic!("Left shift (by u8) not implemented.")
	}
}

impl Shl<u16> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u16) -> BigInt {
		panic!("Left shift (by u16) not implemented.")
	}
}

impl Shl<u32> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u32) -> BigInt {
		panic!("Left shift (by u32) not implemented.")
	}
}
