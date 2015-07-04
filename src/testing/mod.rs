macro_rules! op_test {
    ($name:ident, $operator:tt, $a:tt, $b:tt, $c:tt) => (
        #[test]
        fn $name () {
            let a:BigInt = create_bigint_from_string(stringify!($a)).unwrap();
            let b:BigInt = create_bigint_from_string(stringify!($b)).unwrap();
            let c:BigInt = create_bigint_from_string(stringify!($c)).unwrap();
            println!("{}{}{}?={}", a, stringify!($operator), b, c);
            assert!((a $operator b).compare(&c) == 0);
        }
    )
}

pub mod basic_add_test;
pub mod basic_bitor_test;
pub mod basic_bitxor_test;
pub mod basic_rem_test;
pub mod basic_shl_test;
pub mod basic_shr_test;
pub mod basic_sub_test;
pub mod basic_negation_test;
pub mod general_tests;


