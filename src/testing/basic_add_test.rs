extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_operator_add_positive_positive() {
    if let Ok(v) = create_bigint_from_string("0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x6") {
            let b:BigInt = v;
            let c = a + b;
            assert!(c.data[0] == 11);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_add_positive_negative_positiveresult() {
    if let Ok(v) = create_bigint_from_string("0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x4") {
            let b:BigInt = v;
            let c = a + b;
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
fn test_operator_add_positive_negative_negativeresult() {
    if let Ok(v) = create_bigint_from_string("0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x6") {
            let b:BigInt = v;
            let c = a + b;
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
fn test_operator_add_negative_positive() {
    if let Ok(v) = create_bigint_from_string("-0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x6") {
            let b:BigInt = v;
            let c = a + b;
            assert!(c.data[0] == 1);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_operator_add_negative_negative() {
    if let Ok(v) = create_bigint_from_string("-0x5") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("-0x6") {
            let b:BigInt = v;
            let c = a + b;
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
fn test_basic_add_positive_positive_nocarry() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("0x7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("0x7");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 14);
}

#[test]
fn test_basic_add_positive_negative_nocarry_positiveresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("0x7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-0x3");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 4);
}

#[test]
fn test_basic_add_positive_negative_nocarry_negativeresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("0x7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-0x8");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 1);
}

#[test]
fn test_basic_add_negative_positive_nocarry_positiveresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-0x7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("0x8");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 1);
}

#[test]
fn test_basic_add_negative_positive_nocarry_negativeresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-0x7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("0x3");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 4);
}    

#[test]
fn test_basic_add_negative_negative_nocarry() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-0x3");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-0x3");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 6);
}

#[test]
fn test_add_length_two_integers() {
    if let Ok(v) = create_bigint_from_string("0xFFFFFFFFFFFFFFFF") {
        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0xFFFFFFFFFFFFFFFF") {
            let b:BigInt = v;
            let c:BigInt = a + b;
            println!("c: {}", c.data[0]);
            println!("c: {}", c.data[1]);
            assert!(c.length == 2);
            assert!(c.data[0] == 0xFFFFFFFFFFFFFFFE);
            assert!(c.data[1] == 0x01);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}

#[test]
fn test_add_length_two_large_integers() {
    if let Ok(v) = create_bigint_from_string("0x2fffffffffffffffd") {
        assert!(v.length == 2);
        assert!(v.data[0] == 0xfffffffffffffffd);
        assert!(v.data[1] == 0x2);

        let a:BigInt = v;
        if let Ok(v) = create_bigint_from_string("0x3fffffffffffffffc") {
            assert!(v.length == 2);
            assert!(v.data[0] == 0xfffffffffffffffc);
            assert!(v.data[1] == 0x3);

            let b:BigInt = v;
            let c:BigInt = a + b;
            println!("c: {}", c);
            println!("c[0]: {:x}", c.data[0]);
            println!("c[1]: {:x}", c.data[1]);
            assert!(c.length == 2);
            assert!(c.data[0] == 0xfffffffffffffff9);
            assert!(c.data[1] == 0x06);
        } else {
            panic!("Failed to initialize from string.");
        }
    } else {
        panic!("Failed to initialize from string.");
    }
}
