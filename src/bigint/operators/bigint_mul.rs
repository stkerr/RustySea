use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;
use std::cmp::*;

fn karatsuba_mul(a_orig: &BigInt, b_orig: &BigInt) -> BigInt {

    let stored_negative;
    if (a_orig.negative && !b_orig.negative) || (!a_orig.negative && b_orig.negative) {
        stored_negative = true;
    }
    else {
        stored_negative = false;
    }
    
    let mut a:BigInt = a_orig.clone();
    let mut b:BigInt = b_orig.clone();
    a.negative = false;
    b.negative = false;


    // From https://en.wikipedia.org/wiki/Karatsuba_algorithm#Pseudocode
    //let ten:BigInt  = create_bigint_from_string("0x10").unwrap();
    let two:BigInt = create_bigint_from_string("0x2").unwrap();

println!("{} x {}\n",a,b);
    if a < two || b < two {
        return BigInt {negative:stored_negative, data: vec![a.data[0] * b.data[0]]};
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
    let z1:BigInt = karatsuba_mul(&(low1 + &high1), &(&low2 + &high2));
    let z2:BigInt = karatsuba_mul(&high1, &high2);

    let mut first:BigInt = &two << (&m2_bi * &two);
    first = first * &z2;

    let mut second:BigInt = two.clone() << &m2_bi;
    second = second * (z1 - z2 - &z0);
    let mut result:BigInt = first + second + z0;

    result.negative = stored_negative;

    return result;
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
        let mut interim:BigInt = karatsuba_mul(self, b);

        if interim.data.len() == 1 && interim.data[0] == 0 {
            interim.negative = false;
        }

        return interim;
    }
}
