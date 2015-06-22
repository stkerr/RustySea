use super::super::super::*;

use std;
use std::ops::*;

impl Mul for BigInt {
	type Output = BigInt;

	fn mul(self, b:BigInt) -> BigInt {
		panic!("Multiply not implemented.")
	}
}
