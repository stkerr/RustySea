use rusty_sea::*;

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