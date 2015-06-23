extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_sub_positive_positive_positiveresult() {
    if let Ok(v) = create_bigint_from_string("7") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("6") {
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
    if let Ok(v) = create_bigint_from_string("5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("6") {
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
    if let Ok(v) = create_bigint_from_string("5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-6") {
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
    if let Ok(v) = create_bigint_from_string("-5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("6") {
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
    if let Ok(v) = create_bigint_from_string("-5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-6") {
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
    if let Ok(v) = create_bigint_from_string("-5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-3") {
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