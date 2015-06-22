use super::super::super::*;

use std;
use std::ops::*;

impl BitAnd for BigInt {
	type Output = BigInt;

	fn bitand(self, b:BigInt) -> BigInt {
		panic!("Bit-And not implemented.");
	}
}

impl BitOr for BigInt {
	type Output = BigInt;

	fn bitor(self, b:BigInt) -> BigInt {
		panic!("Bit-Or not implemented.")
	}
}

impl BitXor for BigInt {
	type Output = BigInt;

	fn bitxor(self, b:BigInt) -> BigInt {
		panic!("Bit-Xor not implemented.")
	}
}

impl Div for BigInt {
	type Output = BigInt;

	fn div(self, b:BigInt) -> BigInt {
		panic!("Divide not implemented.")
	}
}

impl Mul for BigInt {
	type Output = BigInt;

	fn mul(self, b:BigInt) -> BigInt {
		panic!("Multiply not implemented.")
	}
}

impl Neg for BigInt {
	type Output = BigInt;

	fn neg(self) -> BigInt {
		panic!("Negative not implemented.")
	}
}

impl Rem for BigInt {
	type Output = BigInt;

	fn rem(self, b:BigInt) -> BigInt {
		panic!("Remainder (modulo) not implemented.");
	}
}

impl Shl<BigInt> for BigInt {
	type Output = BigInt;

	fn shl(self, b:BigInt) -> BigInt {
		panic!("Left shift (by BigInt) not implemented.")
	}
}

impl Shl<u8> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u8) -> BigInt {
		panic!("Left shift (by u8) not implemented.")
	}
}

impl Shl<u16> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u16) -> BigInt {
		panic!("Left shift (by u16) not implemented.")
	}
}

impl Shl<u32> for BigInt {
	type Output = BigInt;

	fn shl(self, b:u32) -> BigInt {
		panic!("Left shift (by u32) not implemented.")
	}
}

impl Shr<BigInt> for BigInt {
	type Output = BigInt;

	fn shr(self, b:BigInt) -> BigInt {
		panic!("Right shift (by BigInt) not implemented.")
	}
}

impl Shr<u8> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u8) -> BigInt {
		panic!("Right shift (by u8) not implemented.")
	}
}

impl Shr<u16> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u16) -> BigInt {
		panic!("Right shift (by u16) not implemented.")
	}
}

impl Shr<u32> for BigInt {
	type Output = BigInt;

	fn shr(self, b:u32) -> BigInt {
		panic!("Right shift (by u32) not implemented.")
	}
}