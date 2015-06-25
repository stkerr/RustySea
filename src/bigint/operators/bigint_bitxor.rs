use ::bigint::BigInt;

use std;
use std::ops::*;

impl BitXor for BigInt {
	type Output = BigInt;

	fn bitxor(self, b:BigInt) -> BigInt {
		return &self ^ &b;
	}
}

impl<'a> BitXor<&'a BigInt> for BigInt {
	type Output = BigInt;

	fn bitxor(self, b:&'a BigInt) -> BigInt {
		return &self ^ b;
	}
}

impl<'a> BitXor<BigInt> for &'a BigInt {
	type Output = BigInt;

	fn bitxor(self, b: BigInt) -> BigInt {
		return self ^ &b;
	}
}

impl<'a,'b> BitXor<&'a BigInt> for &'b BigInt {
	type Output = BigInt;

	fn bitxor(self, b: &'a BigInt) -> BigInt {
		panic!("BitXor not implemented.");
	}
}
