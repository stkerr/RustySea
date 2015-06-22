use super::super::super::*;

use std;
use std::ops::*;

impl BitXor for BigInt {
	type Output = BigInt;

	fn bitxor(self, b:BigInt) -> BigInt {
		panic!("Bit-Xor not implemented.")
	}
}
