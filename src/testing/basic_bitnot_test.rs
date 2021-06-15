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


unary_test!(basic_bitnot_1, !"0x1" == "-0x2");
unary_test!(basic_bitnot_2, !"-0x3" == "0x2");
unary_test!(basic_bitnot_3, !"-0x56fd" == "0x56fc");
unary_test!(basic_bitnot_4, !"0xfd" == "-0xfe");
unary_test!(basic_bitnot_5, !"-0xfd" == "0xfc");
unary_test!(basic_bitnot_6, !"0x0" == "-0x1");
