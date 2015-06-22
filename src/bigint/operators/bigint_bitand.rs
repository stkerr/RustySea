use super::super::super::*;

use std;
use std::ops::*;

impl BitAnd for BigInt {
	type Output = BigInt;

	fn bitand(self, b:BigInt) -> BigInt {
		panic!("Bit-And not implemented.");
	}
}
