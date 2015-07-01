extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_bitand() {
    if let Ok(v) = create_bigint_from_string("8") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("F") {
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
    if let Ok(v) = create_bigint_from_string("8007") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("7") {
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
    if let Ok(v) = create_bigint_from_string("8007FFFFFFFFFFFFFFF0") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("7000000000000000F") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("70000000000000000") {
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
    if let Ok(v) = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("F0F0F0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("F0F0F0F0F0F0F0F0F0F0") {
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

#[test]
fn test_operator_bitand_length2_unequallengths() {
    if let Ok(v) = create_bigint_from_string("0F0F0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        print_bigint(&a);
        if let Ok(v) = create_bigint_from_string("F4FFF0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            print_bigint(&b);
            if let Ok(v) = create_bigint_from_string("F4FFF0F0F0F0F0F0F0F0") {
                let c:BigInt = v;
                let d = a & b;
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