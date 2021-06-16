macro_rules! unary_test {
   ($name:ident, $operator:tt $a:tt == $c:tt) => (

        #[test]
        fn $name () {
            println!("Using {}, {}, and {}", 
                $a, 
                $c, 
                stringify!($operator)
            );

            let mut a_val = String::new();
            a_val.push_str(stringify!($a));
            a_val.remove(0);
            a_val.pop();
            let a_val_str = &a_val[..];

            let mut c_val = String::new();
            c_val.push_str(stringify!($c));
            c_val.remove(0);
            c_val.pop();
            let c_val_str = &c_val[..];

            let a:BigInt = create_bigint_from_string(a_val_str).unwrap();
            let c:BigInt = create_bigint_from_string(c_val_str).unwrap();
            println!("Test: {} {} ?= {}", stringify!($operator), &a, c);
            println!("Result: {} {} = {}", stringify!($operator), &a, $operator &a);
            assert!(($operator a).compare(&c) == 0);
        }
    ) 
}

macro_rules! op_test {
    ($name:ident, $a:tt $operator:tt $b:tt == $c:tt) => (

        #[test]
        fn $name () {
            println!("Using {}, {}, {}, and {}", 
                $a, 
                $b, 
                $c, 
                stringify!($operator)
            );

            let mut a_val = String::new();
            a_val.push_str(stringify!($a));
            a_val.remove(0);
            a_val.pop();
            let a_val_str = &a_val[..];


            let mut b_val = String::new();
            b_val.push_str(stringify!($b));
            b_val.remove(0);
            b_val.pop();
            let b_val_str = &b_val[..];

            let mut c_val = String::new();
            c_val.push_str(stringify!($c));
            c_val.remove(0);
            c_val.pop();
            let c_val_str = &c_val[..];


            let a:BigInt = create_bigint_from_string(a_val_str).unwrap();
            let b:BigInt = create_bigint_from_string(b_val_str).unwrap();
            let c:BigInt = create_bigint_from_string(c_val_str).unwrap();
            println!("Test: {} {} {} ?= {}", &a, stringify!($operator), &b, c);
            println!("Result: {} {} {} = {}", &a, stringify!($operator), &b, &a $operator &b);
            assert!((a $operator b).compare(&c) == 0);
        }
    )
}

macro_rules! op_test_eq {
    ($name:ident, $a:tt $operator:tt $b:tt == $c:tt) => (

        #[test]
        fn $name () {
            println!("Using {}, {}, {}, and {}", 
                $a, 
                $b, 
                $c, 
                stringify!($operator)
            );

            let mut a_val = String::new();
            a_val.push_str(stringify!($a));
            a_val.remove(0);
            a_val.pop();
            let a_val_str = &a_val[..];


            let mut b_val = String::new();
            b_val.push_str(stringify!($b));
            b_val.remove(0);
            b_val.pop();
            let b_val_str = &b_val[..];

            let a:BigInt = create_bigint_from_string(a_val_str).unwrap();
            let b:BigInt = create_bigint_from_string(b_val_str).unwrap();
            println!("Test: {} {} {} ?= {}", &a, stringify!($operator), &b, $c);
            println!("Result: {} {} {} = {}", &a, stringify!($operator), &b, &a $operator &b);
            assert!((a $operator b) == ($c));
        }
    )
}

pub mod basic_add_test;
pub mod basic_bitand_test;
pub mod basic_bitnot_test;
pub mod basic_bitor_test;
pub mod basic_bitxor_test;
pub mod basic_mul_test;
pub mod basic_rem_test;
pub mod basic_shl_test;
pub mod basic_shr_test;
pub mod basic_sub_test;
pub mod basic_negation_test;
pub mod basic_internal_operators;
pub mod general_tests;
pub mod dynamic_tests;
pub mod basic_op_cmp_test;
