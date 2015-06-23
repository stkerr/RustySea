extern crate rusty_sea;

fn main() {
    println!("Hello, world!");

    // Create some integers. Make them overflow.
    let a_result = rusty_sea::bigint::utilities::create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let a:rusty_sea::bigint::BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let b_result = rusty_sea::bigint::utilities::create_bigint_from_string("FFFFFFFFFFFFFFFF");
    let b:rusty_sea::bigint::BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    print!("{} + {}",&a,&b);
    let c:rusty_sea::bigint::BigInt = a+b;
    
    print!(" = {}", c);

    // println!("{} + {} = {}", a, b, a*c);
    // rusty_sea::bigint::utilities::print_bigint(&c);

    // let d = a & b;
    // println!("Hello {}", d);
    
/*
    let mut d = match c {
        Ok(v) => v.add(&b),
        Err(e) => panic!(e)
    };

    let (e, f, g) = signed_add_with_carry(0xFFFFFFFFFFFFFFFF, true, 0xFFFFFFFFFFFFFFFF, true);
    println!("signed a + b = {}. Carry/Borrow: {}. Negative: {}", e, f, g);
    // let (h, i, j) = signed_add_with_carry(0xFFFFFFFFFFFFFFFF, false, 0xFFFFFFFFFFFFFFFF, false);

    // -1 - 0xFFFFFFFFFFFFFFFF = -10000000000000000 (it did a borrow)
    let (h, i, j) = signed_add_with_carry(0x1, true, 0xFFFFFFFFFFFFFFFF, true);
    println!("-1 + (-0xFFFFFFFFFFFFFFFF) = {:x}. Carry/Borrow: {}. Negative: {}", h, i, j);

    // Print the results
    // print_bigint(&a);
    // print_bigint(&b);
    // print_bigint(&c);
    // print_bigint(&d);

    // Do 5+10=15
    let one_result = create_bigint_from_string("1");
    let one = match one_result { Ok(v) => v, Err(e) => panic!(e)};
    let five_result = create_bigint_from_string("5");
    let five = match five_result { Ok(v) => v, Err(e) => panic!(e)};
    let ten_result = create_bigint_from_string("10");
    let ten = match ten_result { Ok(v) => v, Err(e) => panic!(e)};
    let fifteen = five.add(&ten);
    print_bigint(&fifteen);

    println!("1 ?= 5 => {}", one.compare_ignore_sign(&five));
    println!("Larger ?= large => {}", c.compare_ignore_sign(&b));
    println!("large ?= larger => {}", b.compare_ignore_sign(&c));
    println!("larger ?= larger => {}", c.compare_ignore_sign(&c));

    println!("Larger ?= large => {}", c.compare(&b));
    println!("large ?= larger => {}", b.compare(&c));
    println!("larger ?= larger => {}", c.compare(&c));

    // Do some shifts
    let seven_result = create_bigint_from_string("7");
    let seven = match seven_result { Ok(v) => v, Err(e) => panic!(e)};
    let shifted:Result<BigInt, Error> = seven.left_shift(&one);
    match shifted {
        Ok(v) => print_bigint(&v),
        Err(e) => println!("Error {:?}", e)
    }
    
    // Try some adds with negative numbers
    println!("--- Addition & subtraction basic tests ----");
    println!("7 + (-1) ="); print_bigint(&create_bigint_from_string("7").add(&create_bigint_from_string("-1")));
    println!("(-7) + 1 ="); print_bigint(&create_bigint_from_string("-7").add(&create_bigint_from_string("1")));
    println!("(-7) + (-1) ="); print_bigint(&create_bigint_from_string("-7").add(&create_bigint_from_string("-1")));
    println!("7 - 1 ="); print_bigint(&create_bigint_from_string("7").subtract(&create_bigint_from_string("1")));
    println!("7 - (-1) ="); print_bigint(&create_bigint_from_string("7").subtract(&create_bigint_from_string("-1")));
    println!("(-7) - (-1) ="); print_bigint(&create_bigint_from_string("-7").subtract(&create_bigint_from_string("-1")));
    println!("(-7) - 1 ="); print_bigint(&create_bigint_from_string("-7").subtract(&create_bigint_from_string("1")));

    println!("Larger addition & subtraction tests ----");
    let a3 = a.add(&a).add(&a);
    let a2 = a.add(&a);
    println!("a2:");print_bigint(&a2);
    println!("a3:");print_bigint(&a3);
    println!("a3 + a2 = "); print_bigint(&a3.add(&a2));
*/
}



// Run testing
#[cfg(test)]
pub mod testing;
