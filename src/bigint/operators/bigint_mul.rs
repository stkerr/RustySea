use ::bigint::BigInt;

use std;
use std::ops::Mul;

impl Mul for BigInt {
	type Output = BigInt;

	fn mul(self, b:BigInt) -> BigInt {
		panic!("Multiply not implemented.")
	}
}
