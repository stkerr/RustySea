extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn test_string_parsing() {
    let mut a:Result<BigInt,Error>;
    a = create_bigint_from_string("3");
    match a {
        Ok(v) => {
            assert!(v.length == 1);
            assert!(v.data[0] == 3);
            assert!(v.negative == false);
        }
        Err(e) => panic!(e)
    };

    a = create_bigint_from_string("-3");
    match a {
        Ok(v) => {
            assert!(v.length == 1);
            assert!(v.data[0] == 3);
            assert!(v.negative == true);
        }
        Err(e) => panic!(e)
    }
}

#[test]
fn test_string_parsing_two_characters() {
    let a:Result<BigInt,Error> = create_bigint_from_string("350");
    match a {
        Ok(v) => {
            assert!(v.length == 1);
            assert!(v.data[0] == 0x350);
            assert!(v.negative == false);
        },
        Err(e) => panic!(e)
    }
}

#[test]
fn test_string_parsing_empty_string() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("");
    match a_result {
        Ok(v) => panic!(v),
        Err(e) => e
    };
}

#[test]
fn test_string_parsing_negativeonly_string() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-");
    match a_result {
        Ok(v) => panic!(v),
        Err(e) => e
    };
}

#[test]
fn test_string_parsing_invalid_string() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("Hello world!");
    match a_result {
        Ok(v) => panic!(v),
        Err(e) => e
    };
}

#[test]
fn test_string_parsing_zero() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("0");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 0);
    assert!(a.negative == false);
}

#[test]
fn test_string_parsing_negative_zero() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-0");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 0);
    assert!(a.negative == true);
}

#[test]
fn test_string_parsing_leadingzeros() {
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("0000");
    let mut a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 0);
    assert!(a.negative == false);

    a_result = create_bigint_from_string("-00003");
    a = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 3);
    assert!(a.negative == true);

    a_result = create_bigint_from_string("00003");
    a = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 3);
    assert!(a.negative == false);
}

#[test]
fn test_string_parsing_negativezero_length_two() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-00000000000000000000000000000000000000000000000000000000000000");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 1);
    assert!(a.data[0] == 0);
    assert!(a.negative == true);
}

#[test]
fn test_string_parsing_length_two() {
    let mut a_result:Result<BigInt,Error> = create_bigint_from_string("3FFFFFFFFFFFFFFFF");
    let mut a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 2);
    assert!(a.data[0] == 0xFFFFFFFFFFFFFFFF);
    assert!(a.data[1] == 3);
    assert!(a.negative == false);

    a_result = create_bigint_from_string("-3FFFFFFFFFFFFFFFF");
    a = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    assert!(a.length == 2);
    assert!(a.data[0] == 0xFFFFFFFFFFFFFFFF);
    assert!(a.data[1] == 3);
    assert!(a.negative == true);

}


#[test]
fn test_basic_sub_positive_positive_nocarry_positiveresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("1");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 6);
}

#[test]
fn test_basic_sub_positive_positive_nocarry_negativeresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("8");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 1);
}

#[test]
fn test_basic_sub_positive_negative_nocarry_positiveresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-3");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 10);
}

#[test]
fn test_basic_sub_negative_positive_nocarry() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-7");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("8");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 15);
}    

#[test]
fn test_basic_sub_negative_negative_nocarry_positiveresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-3");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-4");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == false);
    assert!(c.data[0] == 1);
}

#[test]
fn test_basic_sub_negative_negative_nocarry_negativeresult() {
    let a_result:Result<BigInt,Error> = create_bigint_from_string("-3");
    let a:BigInt = match a_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let b_result:Result<BigInt,Error> = create_bigint_from_string("-2");
    let b:BigInt = match b_result {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let c:BigInt = a.subtract(&b);
    assert!(c.length == 1);
    assert!(c.negative == true);
    assert!(c.data[0] == 1);
}
