extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_bitor() {
    if let Ok(v) = create_bigint_from_string("0x8") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0xF") {
                let c:BigInt = v;
                let d = a | b;
                println!("c: {}", c.data[0]);
                println!("d: {}", d.data[0]);
                assert!(c.compare(&d) == 0);
            } else {
                panic!("Failed to initialize from string.");
            }
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_bitor_unequallengths() {
    if let Ok(v) = create_bigint_from_string("0x8007") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0x8007") {
                let c:BigInt = v;
                let d = a | b;
                println!("c: {}", c.data[0]);
                println!("d: {}", d.data[0]);
                assert!(c.data[0] == d.data[0]);
            } else {
                panic!("Failed to initialize from string.");
            }
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_bitor_length2() {
    if let Ok(v) = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0xF0F0F0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFF") {
                let c:BigInt = v;
                let d = a | b;
                println!("c[0]: {}", c.data[0]);
                println!("c[1]: {}", c.data[1]);
                println!("d[0]: {}", d.data[0]);
                println!("d[1]: {}", d.data[1]);

                assert!(c.compare(&d) == 0);
            } else {
                panic!("Failed to initialize from string.");
            }
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_bitor_length2_unequallengths() {
    if let Ok(v) = create_bigint_from_string("0x0F0F0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        print_bigint(&a);
        if let Ok(v) = create_bigint_from_string("0xF4FFF0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            print_bigint(&b);
            if let Ok(v) = create_bigint_from_string("0x0F0F0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFFFFFF") {
                let c:BigInt = v;
                let d = a | b;
                println!("c[0]: {:16x}", c.data[0]);
                println!("c[1]: {:16x}", c.data[1]);
                println!("c[2]: {:16x}", c.data[2]);
                println!("d[0]: {:16x}", d.data[0]);
                println!("d[1]: {:16x}", d.data[1]);
                println!("d[2]: {:16x}", d.data[2]);

                assert!(c.compare(&d) == 0);
            } else {
                panic!("Failed to initialize from string.");
            }
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

op_test!(basic_bitor_1, "0x1" | "0x1" == "0x1");
op_test!(basic_bitor_2, "0x1" | "0x2" == "0x3");
op_test!(basic_bitor_3, "0x1" | "0x3" == "0x3");
op_test!(basic_bitor_4, "0x1" | "0x4" == "0x5");
op_test!(basic_bitor_5, "0xF0F0F0F0F0F0F0F0" | "0x0F0F0F0F0F0F0F0F" == "0xFFFFFFFFFFFFFFFF");
op_test!(basic_bitor_6, "0xF0F0F0F0F0F0F0F0" | "0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F" == "0x0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFF");
op_test!(basic_bitor_7, "0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F" | "0xF0F0F0F0F0F0F0F0" == "0x0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFF");

op_test!(basic_bitor_10, "-0x1" | "0x1" == "-0x1");
op_test!(basic_bitor_11, "-0x1" | "0x2" == "-0x1");
op_test!(basic_bitor_12, "-0x1" | "0x3" == "-0x1");
op_test!(basic_bitor_13, "-0x1" | "0x4" == "-0x1");