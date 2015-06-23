use ::bigint::BigInt;

use std;
use std::ops::*;

impl Div for BigInt {
	type Output = BigInt;

	fn div(self, b:BigInt) -> BigInt {
		panic!("Divide not implemented.")
	}
}
