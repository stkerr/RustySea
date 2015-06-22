use super::super::super::*;

use std;
use std::ops::*;

impl Shr<BigInt> for BigInt {
	type Output = BigInt;

	fn shr(self, b:BigInt) -> BigInt {
		panic!("Right shift (by BigInt) not implemented.")
	}
}

impl Shr<u8> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u8) -> BigInt {
		panic!("Right shift (by u8) not implemented.")
	}
}

impl Shr<u16> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u16) -> BigInt {
		panic!("Right shift (by u16) not implemented.")
	}
}

impl Shr<u32> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u32) -> BigInt {
		panic!("Right shift (by u32) not implemented.")
	}
}
