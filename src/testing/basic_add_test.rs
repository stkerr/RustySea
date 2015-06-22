use rusty_sea::*;

#[test]
fn test_basic_add_positive_positive_nocarry() {
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("7");
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
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-3");
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
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-8");
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
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("8");
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
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("3");
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
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("-3");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let mut b_result:Result<BigInt,Error> = create_bigint_from_string("-3");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.add(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 6);
}