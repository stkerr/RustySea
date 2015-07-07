use ::bigint::BigInt;
use ::bigint::utilities::*;

use std::ops::*;

impl Rem for BigInt {
    type Output = BigInt;

    fn rem(self, b:BigInt) -> BigInt {
        return &self % &b;
    }
}

impl<'a> Rem<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, b:&'a BigInt) -> BigInt {
        return &self % b;
    }
}

impl<'a> Rem<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn rem(self, b: BigInt) -> BigInt {
        return self % &b;
    }
}

impl<'a,'b> Rem<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn rem(self, b: &'a BigInt) -> BigInt {
        if self.negative {
            let mut self_clone = self.clone();
            self_clone.negative = false;

            let mut temp = self_clone % b;
            temp = temp + b;

            return temp;
        } else {
            /*
             * Calculat the remainder by repeatedly squaring the
             * modulus until it is larger than the value in question.
             *
             * At that point, subtratract the previous modulus value (the
             * multiply squared one) and a single multiple of the modulus
             * from the original value untl it is less than the modulus
             */
            let one:BigInt = create_bigint_from_string("0x1").unwrap();
            let mut self_clone = self.clone();
            let mut mod_clone = b.clone();
            let mut prev_mod_clone:BigInt = b.clone();
            let mut count:u8 = 0;

            // This is the reduction loop, so that we reduce our problem
            // to a problem simpler than the original. Namely, rather than
            // x % p, where x >> p, we attempt to reduce it to x' % p', where
            // x' >>! p' (not much larger than).
            if mod_clone.compare(&one) == 0 {
                // We can't do the reduction loop, since 1*1=1, so will
                // be infinite. Since we are doing modulus 1, we can
                // just return zero though.
                return create_bigint_from_string("0x0").unwrap();
            }

            while mod_clone.compare(&self_clone) < 0 {
                prev_mod_clone = mod_clone.clone();

                mod_clone = &mod_clone * &mod_clone;
            }

            println!("Starting as {}", self_clone);
            println!("prev_mod_clone {}", prev_mod_clone);
            if self_clone.compare(&prev_mod_clone) > 0 {
                self_clone = self_clone - &prev_mod_clone;
            }

            let mut mod_multiples = vec![b.clone()];
            let mut mod_clone = b.clone();
            while mod_clone.compare(&self_clone) < 0 {
                mod_multiples.push(mod_clone.clone());
                prev_mod_clone = mod_clone.clone();

                // If we are doing modulo N, we can do modulo 2N 
                // and still get the correct result (assuming we reduce again)
                mod_clone = &mod_clone << &one;
                println!("mod_clone: {}", mod_clone);
            }

            for i in mod_multiples.clone() {
                println!("Reductor: {}", i);
            }

            let mut count = 0;
            while mod_multiples.len() > 0 {
                let reductor = mod_multiples.pop().unwrap();

                // Reduce by a reductor that is probably larger than
                // the modulus for speed.
                while reductor.compare(&self_clone) <= 0 {
                    println!("Now {}", self_clone);
                    self_clone = self_clone - &reductor; 
                }
            }
            
            let mut count = 0;
            while self_clone.compare(&b) >= 0 {
                // This is the final, slowest reduction loop
                self_clone = self_clone - b;

                count = count + 1;
                if count == 5 {
                    panic!("Done.");
                }
            }

            println!("prev_mod_clone: {}", prev_mod_clone);
            println!("b: {}", b);
            println!("prev_mod_clone ? b: {}", prev_mod_clone.compare(&b));
            println!("Result: {}", self_clone);
            
            return self_clone;
        }
    }
}
