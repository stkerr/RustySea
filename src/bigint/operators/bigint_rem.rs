use ::bigint::BigInt;

use std;
use std::ops::*;

impl Rem for BigInt {
	type Output = BigInt;

	fn rem(self, b:BigInt) -> BigInt {
		return &self & &b;
	}
}

impl<'a> Rem<&'a BigInt> for BigInt {
	type Output = BigInt;

	fn rem(self, b:&'a BigInt) -> BigInt {
		return &self & b;
	}
}

impl<'a> Rem<BigInt> for &'a BigInt {
	type Output = BigInt;

	fn rem(self, b: BigInt) -> BigInt {
		return self & &b;
	}
}

impl<'a,'b> Rem<&'a BigInt> for &'b BigInt {
	type Output = BigInt;

	fn rem(self, b: &'a BigInt) -> BigInt {
		panic!("Rem not implemented.");
	}
}
