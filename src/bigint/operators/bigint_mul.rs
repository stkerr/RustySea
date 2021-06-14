use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;
use std::cmp::*;


fn gradeschool_mul(a: &BigInt, b: &BigInt) -> BigInt {
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

        if a.compare(&zero) == 0 || b.compare(&zero) == 0 {
            // Hardcode multiply by zero
            return zero.clone();
        }

        let one:BigInt = create_bigint_from_string("0x1").unwrap();
        let mut c:BigInt = zero.clone();
        let mut a_copy:BigInt = a.clone();
        let mut b_copy:BigInt = b.to_owned();
        while (b_copy.compare(&zero)) != 0 {
            if (&b_copy & &one).compare(&zero) != 0 {
                c = &c + &a_copy;
            }
            a_copy = &a_copy << &one;
            b_copy = &b_copy >> &one;
        }
        if (a.negative && !b.negative) || (!a.negative && b.negative) {
            c.negative = true;
        } else {
            c.negative = false;
        }

        return c;
    }


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

    let max_size:BigInt = create_bigint_from_string("0xffffffffffffffff").unwrap();

println!("mult: {} x {}\n",a,b);
println!("max:  {}\n", max_size);
    if a <= max_size && b <= max_size {
        return gradeschool_mul(&a_orig, &b_orig);
    }

    /* Calculates the size of the numbers. */
    //m = min(size_base10(num1), size_base10(num2))
    let m:usize = max(a.data.len(), b.data.len());
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

    print_bigint(&low1);
    print_bigint(&high1);
    print_bigint(&low2);
    print_bigint(&high2);
    println!("");
    /*
    high1, low1 = split_at(num1, m2)
    high2, low2 = split_at(num2, m2)
    */

    /* 3 calls made to numbers approximately half the size. */
    println!("Getting z0");
    let z0:BigInt = karatsuba_mul(&low1, &low2);
    println!("Getting z1");
    let z1:BigInt = karatsuba_mul(&(low1 + &high1), &(&low2 + &high2));
    println!("Getting z2");
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
