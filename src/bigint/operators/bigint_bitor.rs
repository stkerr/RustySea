use super::super::super::*;

use std;
use std::ops::*;

impl BitOr for BigInt {
	type Output = BigInt;

	fn bitor(self, b:BigInt) -> BigInt {
		panic!("Bit-Or not implemented.")
	}
}
