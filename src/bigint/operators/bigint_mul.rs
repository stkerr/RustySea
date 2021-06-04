use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;
use std::cmp::*;


fn karatsuba_mul(a: &BigInt, b: &BigInt) -> BigInt {
    // From https://en.wikipedia.org/wiki/Karatsuba_algorithm#Pseudocode
    //let ten:BigInt  = create_bigint_from_string("0x10").unwrap();
    let two:BigInt  = create_bigint_from_string("0x2").unwrap();
    if a < &two || b < &two {
        return a * b;
    }

    /* Calculates the size of the numbers. */
    //m = min(size_base10(num1), size_base10(num2))
    let m:usize = min(a.data.len(), b.data.len());
    let m2:usize = m / 2; 
    /* m2 = ceil(m / 2) will also work */

    //let m_bi:BigInt = BigInt { negative:false, data: vec![m as u64]};
    let m2_bi:BigInt = BigInt { negative:false, data: vec![m2 as u64]};

    /* Split the digit sequences in the middle. */
    let high1:BigInt = BigInt {
        negative: a.negative,
        data: a.data.get(0..m2).unwrap().to_vec()
    };
    let low1:BigInt = BigInt {
        negative: a.negative,
        data: a.data.get(m2..a.data.len()).unwrap().to_vec()
    };

    let high2:BigInt = BigInt {
        negative: b.negative,
        data: b.data.get(0..m2).unwrap().to_vec()
    };
    let low2:BigInt = BigInt {
        negative: b.negative,
        data: b.data.get(m2..b.data.len()).unwrap().to_vec()
    };

    /*
    high1, low1 = split_at(num1, m2)
    high2, low2 = split_at(num2, m2)
    */

    /* 3 calls made to numbers approximately half the size. */
    let z0:BigInt = karatsuba_mul(&low1, &low2);
    let z1:BigInt = karatsuba_mul(&(low1 + high1.clone()), &(low2 + high2.clone()));
    let z2:BigInt = karatsuba_mul(&high1, &high2);

    let mut first:BigInt = &two << (&m2_bi * &two);
    first = first * &z2;

    let mut second:BigInt = two.clone() << &m2_bi;
    second = second * (z1 - z2 - &z0);
    return first + second + &z0;
}


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
        /*
         * From Wikipedia
        c = 0

        while b ≠ 0
            if (b and 1) ≠ 0
                c = c + a
            left shift a by 1
            right shift b by 1

        return c
        */
        let zero:BigInt  = create_bigint_from_string("0x0").unwrap();
        
        if self.compare(&zero) == 0 || b.compare(&zero) == 0 {
            // Hardcode multiply by zero
            return zero.clone();
        }

        let one:BigInt = create_bigint_from_string("0x1").unwrap();
        let mut c:BigInt = zero.clone();
        let mut a_copy:BigInt = self.clone();
        let mut b_copy:BigInt = b.to_owned();
        while (b_copy.compare(&zero)) != 0 {
            if (&b_copy & &one).compare(&zero) != 0 {
                c = &c + &a_copy;
            }
            a_copy = &a_copy << &one;
            b_copy = &b_copy >> &one;
        }
        if (self.negative && !b.negative) || (!self.negative && b.negative) {
            c.negative = true;
        } else {
            c.negative = false;
        }

        return c;
    }
}
