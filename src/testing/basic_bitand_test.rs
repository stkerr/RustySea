extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_bitand() {
    if let Ok(v) = create_bigint_from_string("0x8") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0x0") {
                let c:BigInt = v;
                let d = a & b;
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
fn test_operator_bitand_unequallengths() {
    if let Ok(v) = create_bigint_from_string("0x8007") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0x7") {
                let c:BigInt = v;
                let d = a & b;
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
fn test_operator_bitand_unequallengths_blockboundary() {
    if let Ok(v) = create_bigint_from_string("0x8007FFFFFFFFFFFFFFF0") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x7000000000000000F") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0x70000000000000000") {
                let c:BigInt = v;
                let d = a & b;
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
fn test_operator_bitand_length2() {
    if let Ok(v) = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0xF0F0F0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0xF0F0F0F0F0F0F0F0F0F0") {
                let c:BigInt = v;
                let d = a & b;
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

op_test!(bitand_neg_test_1, "-0x3" & "0x1" == "0x1");
op_test!(bitand_test_1, "0x0F0F0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFFFFFF" & "0xF4FFF0F0F0F0F0F0F0F0" == "0xF4FFF0F0F0F0F0F0F0F0");
op_test!(bitand_test_2, "0x0" & "0x1" == "0x0");
op_test!(bitand_test_3, "-0x1" & "0x1" == "0x1");
op_test!(bitand_test_4, "-0x4" & "0x8" == "0x8");
