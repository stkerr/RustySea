extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_not_one() {
    if let Ok(v) = create_bigint_from_string("0x1") {
        let a:BigInt = !v;
        println!("0x{:x} negative:{}", a.data[0], a.negative);
        assert!(a.data[0] == 2);
        assert!(a.negative == true);
    } else {
        panic!("Failed to initialize from string.");
    }
}