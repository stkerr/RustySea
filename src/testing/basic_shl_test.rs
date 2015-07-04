extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn basic_shl_test_most_basic() {
    let a:BigInt = create_bigint_from_string("1").unwrap();
    let b:BigInt = create_bigint_from_string("1").unwrap();
    let c:BigInt = create_bigint_from_string("2").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_negative_shift() {
    let a:BigInt = create_bigint_from_string("1").unwrap();
    let b:BigInt = create_bigint_from_string("-1").unwrap();
    let c:BigInt = create_bigint_from_string("0").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_basic_variety() {
    let a:BigInt = create_bigint_from_string("1").unwrap();
    let b:BigInt = create_bigint_from_string("3").unwrap();
    let c:BigInt = create_bigint_from_string("8").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary() {
    let a:BigInt = create_bigint_from_string("1").unwrap();
    let b:BigInt = create_bigint_from_string("40").unwrap();
    let c:BigInt = create_bigint_from_string("10000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_larger_value() {
    let a:BigInt = create_bigint_from_string("f00d").unwrap();
    let b:BigInt = create_bigint_from_string("40").unwrap();
    let c:BigInt = create_bigint_from_string("f00d0000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_length_two() {
    let a:BigInt = create_bigint_from_string("f00df00df00df00dbeefbeef").unwrap();
    let b:BigInt = create_bigint_from_string("40").unwrap();
    let c:BigInt = create_bigint_from_string("f00df00df00df00dbeefbeef0000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_length_three() {
    let a:BigInt = create_bigint_from_string("f00df00df00df00dbeefbeef").unwrap();
    let b:BigInt = create_bigint_from_string("60").unwrap();
    let c:BigInt = create_bigint_from_string("f00df00df00df00dbeefbeef000000000000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}
