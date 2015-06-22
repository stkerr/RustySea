use super::super::super::*;

use std;
use std::ops::*;

impl Neg for BigInt {
	type Output = BigInt;

	fn neg(self) -> BigInt {
		let mut result = self.clone();
		result.negative = !self.negative;
		return result;
	}
}
