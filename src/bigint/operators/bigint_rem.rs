use ::bigint::BigInt;

use std;
use std::ops::*;

impl Rem for BigInt {
	type Output = BigInt;

	fn rem(self, b:BigInt) -> BigInt {
		panic!("Remainder (modulo) not implemented.");
	}
}
