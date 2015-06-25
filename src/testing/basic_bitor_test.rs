extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_bitor() {
    if let Ok(v) = create_bigint_from_string("8") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("F") {
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
fn test_operator_bitor_unequallengths() {
    if let Ok(v) = create_bigint_from_string("8007") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("7") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("8007") {
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
    if let Ok(v) = create_bigint_from_string("0F0F0F0F0F0F0F0F0F0F") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("F0F0F0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFF") {
                let c:BigInt = v;
                let d = a | b;
                println!("c[0]: {}", c.data[0]);
                println!("c[1]: {}", c.data[1]);
                println!("d[0]: {}", d.data[0]);
                println!("d[1]: {}", d.data[1]);

                assert!(c.data[0] == d.data[0]);
                assert!(c.data[1] == d.data[1]);
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
    if let Ok(v) = create_bigint_from_string("0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("F0F0F0F0F0F0F0F0F0F0") {
            let b:BigInt = v;
            if let Ok(v) = create_bigint_from_string("0F0F0F0F0F0F0F0F0F0FFFFFFFFFFFFFFFFFFFFF") {
                let c:BigInt = v;
                let d = a | b;
                println!("c[0]: {}", c.data[0]);
                println!("c[1]: {}", c.data[1]);
                println!("d[0]: {}", d.data[0]);
                println!("d[1]: {}", d.data[1]);

                assert!(c.data[0] == d.data[0]);
                assert!(c.data[1] == d.data[1]);
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
