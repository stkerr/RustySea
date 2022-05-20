use crate::bigint::BigInt;
use crate::bigint::utilities::*;

use std::io::*;
use std::ops::*;
use std::cmp::*;
use std::convert::TryInto;

fn doubleadd_mul(a: &BigInt, b: &BigInt) -> BigInt {
    /*
     * 
    def mult(a, b):
        add_buffer = 0
        top = a
        while b > 1:
            digit = b % 2
            if digit == 1:
                add_buffer = add_buffer + top

            top = top << 1
            b = b >> 1
        return top + add_buffer 
     */

    
    let zero:BigInt  = create_bigint_from_string("0x0").unwrap();

    print_bigint(a);
    print_bigint(b);
    print_bigint(&zero);

    println!("a?=0 : {}", a == &zero);
    println!("b?=0 : {}", b == &zero);
    if a.compare(&zero) == 0 || b.compare(&zero) == 0 || a.data.len() == 0 || b.data.len() == 0 {
        // Hardcode multiply by zero
        eprintln!("Zero shortcut!");
        std::io::stdout().flush().unwrap();
        return zero.clone();
    }

    let a_copy:u64 = a.data[0];
    let mut b_copy:u64 = b.data[0];

    let mut add_buffer:u64 = 0;
    let mut top:u64 = a_copy;

    let digit:u64 = b_copy % 2;

    while b_copy > 1 {
        println!("In the loop.");

        if digit == 1 {
            add_buffer = add_buffer + top
        }
        top = top << 1;
        b_copy = b_copy >> 1;
    }
    println!("Out of the loop.");
    // TODO: Fix this
    return create_bigint_from_string(&("0x".to_owned() + &((top + add_buffer).to_string()))).unwrap();

}

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

    println!("Grade school math: {} {}", a, b);

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
 
    println!("Stored negative: {}", stored_negative);
    // shortcut zero multiply
    let zero:BigInt  = create_bigint_from_string("0x0").unwrap();
    if a_orig == &zero || b_orig == &zero || a_orig.data.len() == 0 || b_orig.data.len() == 0 {
        println!("Zero shortcut!");
        return zero;
    }

    let mut a:BigInt = a_orig.clone();
    let mut b:BigInt = b_orig.clone();
    a.negative = false;
    b.negative = false;


    // From https://en.wikipedia.org/wiki/Karatsuba_algorithm#Pseudocode

    //let max_size:BigInt = create_bigint_from_string("0xffffffffffffffff").unwrap();
    let max_size:BigInt = create_bigint_from_string("0xffffffffffffff").unwrap();

println!("mult: {} x {}\n",a,b);
println!("max:  {}\n", max_size);
    if a <= max_size || b <= max_size {
        println!("Going doubleadd_mul.");
        return doubleadd_mul(&a_orig, &b_orig);
    }
    println!("Going karatsuba. {} {}", &a, &b);
    /* Calculates the size of the numbers. */
    //m = min(size_base10(num1), size_base10(num2))
    if a.data.len() == 1 && b.data.len() == 1 {
        println!("Got to one data block each!");

        return doubleadd_mul(&a_orig, &b_orig);
    }
    let m:usize = min(a.data.len(), b.data.len());
    let m2:usize = m / 2; 
    /* m2 = ceil(m / 2) will also work */


    /* Split the digit sequences in the middle. */
    let low1:BigInt;
    let high1:BigInt;

    println!("m2: {}", m2);

    low1 = BigInt {
        negative: a.negative,
        data: a.data.get(0..m2).unwrap().to_vec()
    };
    if(m2..a.data.len()).is_empty() {
        high1= BigInt {
            negative: a.negative,
            data: vec![0 as u64]
        };
    }
    else {
        high1= BigInt {
            negative: a.negative,
            data: a.data.get(m2..a.data.len()).unwrap().to_vec()
        };
    }

    let low2:BigInt;
    let high2:BigInt;
    low2 = BigInt {
        negative: b.negative,
        data: b.data.get(0..m2).unwrap().to_vec()
    };
    if (m2..b.data.len()).is_empty() {
        high2 = BigInt {
            negative: b.negative,
            data: vec![0 as u64]
        };
    }
    else {
        high2 = BigInt {
            negative: b.negative,
            data: b.data.get(m2..b.data.len()).unwrap().to_vec()
        };
    }

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
    let z1:BigInt = karatsuba_mul(&(&low1 + &high2), &(&low2 + &high2));
    println!("Getting z2");
    let z2:BigInt = karatsuba_mul(&high1, &high2);

    println!("z0 = {}", z0);
    println!("z1 = {}", z1);
    println!("z2 = {}", z2);

    let mut first:BigInt = BigInt {
        negative:false,
        data:vec![0 as u64; (m2*2+1 as usize).try_into().unwrap()]
    };
    println!("length: {}",m2*2+1);
    first.data[m2*2]=1;
    first = first * &z2;
    println!("first: {}", first);

    let mut second:BigInt = BigInt {
        negative:false,
        data:vec![0 as u64; (m2+1 as usize).try_into().unwrap()]
    };
    second.data[m2]=1;
    second = second * (z1 - z2 - &z0);
    println!("second: {}", second);

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
