extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn basic_bitxor_test() {
    let a:BigInt = create_bigint_from_string("0x0F0F").unwrap();
    let b:BigInt = create_bigint_from_string("0xF0F0").unwrap();
    let c:BigInt = create_bigint_from_string("0xFFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_length2_interblockdifference() {
    let a:BigInt = create_bigint_from_string("0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F").unwrap();
    let b:BigInt = create_bigint_from_string("0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0").unwrap();
    let c:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_length2_fulldifference() {
    let a:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFF0000000000000000").unwrap();
    let b:BigInt = create_bigint_from_string("0x0000000000000000FFFFFFFFFFFFFFFF").unwrap();
    let c:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_unequal_length() {
    let a:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFF").unwrap();
    let b:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    let c:BigInt = create_bigint_from_string("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000000000").unwrap();
    assert!((a^b).compare(&c) == 0);
}

op_test!(basic_bitxor_1, "0xFFFFFFFFFFFFFFFF" ^ "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF" == "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000000000");