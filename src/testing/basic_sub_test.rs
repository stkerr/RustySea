extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_sub_positive_positive_positiveresult() {
    if let Ok(v) = create_bigint_from_string("0x7") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x6") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 1);
            assert!(c.negative == false);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_sub_positive_positive_negativeresult() {
    if let Ok(v) = create_bigint_from_string("0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x6") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 1);
            assert!(c.negative == true);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_sub_positive_negative() {
    if let Ok(v) = create_bigint_from_string("0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x6") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 11);
            assert!(c.negative == false);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_sub_negative_positive() {
    if let Ok(v) = create_bigint_from_string("-0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x6") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 11);
            assert!(c.negative == true);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_sub_negative_negative_positiveresult() {
    if let Ok(v) = create_bigint_from_string("-0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x6") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 1);
            assert!(c.negative == false);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_sub_negative_negative_negativeresult() {
    if let Ok(v) = create_bigint_from_string("-0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x3") {
            let b:BigInt = v;
            let c = a - b;
            assert!(c.data[0] == 2);
            assert!(c.negative == true);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
            panic!("Failed to initialize from string.");
    }
}

op_test!(sub_large_numbers, "0x0000000000000051000000000000030f" - "0x0000010000000000" == "0x50ffffff000000030f");
+op_test!(basic_sub_1, "0x51000000000000030F" - "0x400" == "0x50ffffff000000030f");
+op_test!(basic_sub_2, "0x510" - "0x400" == "0x110");
+op_test!(basic_sub_3, "0x51000000000000030F" - "0x100" == "0x51000000000000020F");
+op_test!(basic_sub_4, "0x51000000000000030F" - "0x30f" == "0x510000000000000000");
+op_test!(basic_sub_5, "0x51000000000000030F" - "0x310" == "0x50ffffffffffffffff");
+op_test!(basic_sub_6, "0x30F" - "0x10" == "0x2FF");
+op_test!(basic_sub_7, "0x10000000000000000" - "0x1" == "0xffffffffffffffff");
