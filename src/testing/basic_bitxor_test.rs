extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

#[test]
fn basic_bitxor_test() {
    let a:BigInt = create_bigint_from_string("0F0F").unwrap();
    let b:BigInt = create_bigint_from_string("F0F0").unwrap();
    let c:BigInt = create_bigint_from_string("FFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_length2_interblockdifference() {
    let a:BigInt = create_bigint_from_string("0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F").unwrap();
    let b:BigInt = create_bigint_from_string("F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0").unwrap();
    let c:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_length2_fulldifference() {
    let a:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFF0000000000000000").unwrap();
    let b:BigInt = create_bigint_from_string("0000000000000000FFFFFFFFFFFFFFFF").unwrap();
    let c:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    assert!((a^b).compare(&c) == 0);
}

#[test]
fn basic_bitxor_test_unequal_length() {
    let a:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFF").unwrap();
    let b:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
    let c:BigInt = create_bigint_from_string("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000000000").unwrap();
    assert!((a^b).compare(&c) == 0);
}
