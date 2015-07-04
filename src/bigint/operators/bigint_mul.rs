use ::bigint::BigInt;

use std::ops::*;

impl Mul for BigInt {
	type Output = BigInt;

	fn mul(self, b:BigInt) -> BigInt {
		return &self * &b;
	}
}

impl<'a> Mul<&'a BigInt> for BigInt {
	type Output = BigInt;

	fn mul(self, b:&'a BigInt) -> BigInt {
		return &self * b;
	}
}

impl<'a> Mul<BigInt> for &'a BigInt {
	type Output = BigInt;

	fn mul(self, b: BigInt) -> BigInt {
		return self * &b;
	}
}

impl<'a,'b> Mul<&'a BigInt> for &'b BigInt {
	type Output = BigInt;

	fn mul(self, b: &'a BigInt) -> BigInt {
		panic!("Mul not implemented for {} * {}.", self, b);
	}
}
