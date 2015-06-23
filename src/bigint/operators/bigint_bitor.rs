use ::bigint::BigInt;

use std;
use std::ops::*;

impl BitOr for BigInt {
	type Output = BigInt;

	fn bitor(self, b:BigInt) -> BigInt {
		panic!("Bit-Or not implemented.")
	}
}
