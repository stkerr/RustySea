extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn basic_shl_test_most_basic() {
    let a:BigInt = create_bigint_from_string("0x1").unwrap();
    let b:BigInt = create_bigint_from_string("0x1").unwrap();
    let c:BigInt = create_bigint_from_string("0x2").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_negative_shift() {
    let a:BigInt = create_bigint_from_string("0x1").unwrap();
    let b:BigInt = create_bigint_from_string("-0x1").unwrap();
    let c:BigInt = create_bigint_from_string("0x0").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_basic_variety() {
    let a:BigInt = create_bigint_from_string("0x1").unwrap();
    let b:BigInt = create_bigint_from_string("0x3").unwrap();
    let c:BigInt = create_bigint_from_string("0x8").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary() {
    let a:BigInt = create_bigint_from_string("0x1").unwrap();
    let b:BigInt = create_bigint_from_string("0x40").unwrap();
    let c:BigInt = create_bigint_from_string("0x10000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_larger_value() {
    let a:BigInt = create_bigint_from_string("0xf00d").unwrap();
    let b:BigInt = create_bigint_from_string("0x40").unwrap();
    let c:BigInt = create_bigint_from_string("0xf00d0000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_length_two() {
    let a:BigInt = create_bigint_from_string("0xf00df00df00df00dbeefbeef").unwrap();
    let b:BigInt = create_bigint_from_string("0x40").unwrap();
    let c:BigInt = create_bigint_from_string("0xf00df00df00df00dbeefbeef0000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

#[test]
fn basic_shl_test_across_boundary_length_three() {
    let a:BigInt = create_bigint_from_string("0xf00df00df00df00dbeefbeef").unwrap();
    let b:BigInt = create_bigint_from_string("0x60").unwrap();
    let c:BigInt = create_bigint_from_string("0xf00df00df00df00dbeefbeef000000000000000000000000").unwrap();
    assert!((a<<b).compare(&c) == 0);
}

op_test!(shl_block_boundary_shift, "0x8000000000000000" << "0x1" == "0x10000000000000000");
op_test!(shl_one_across, "0x10" << "0x1" == "0x20");

