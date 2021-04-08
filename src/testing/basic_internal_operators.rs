extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

macro_rules! func_test_tuple {
    ($testname: tt, $function: tt, $a:expr, $b:expr, $result:expr, $carry:expr) => (
        #[test]
        fn $testname() {
            println!("$function ({:x}, {:x}) ?= ({:x}, {:x})",
                $a,
                $b,
                $result,
                $carry
            );
            let (calc_result, calc_carry) = $function($a, $b);
            println!("\t= ({:x},{:x}\n", calc_result, calc_carry);
            assert!($result == calc_result);
            assert!($carry == calc_carry);
        }
    )
}


macro_rules! func_test_triple {
    ($testname: tt, $function: tt, $a:expr, $b:expr, $c:expr, $d:expr, $result:expr, $carry:expr, $signed:expr) => (
        #[test]
        fn $testname() {
            println!("$function ({:x}, {}, {:x}, {}) ?= ({:x}, {:x}, {})",
                $a,
                $b,
                $c,
                $d,
                $result,
                $carry,
                $signed
            );
            let (calc_result, calc_carry, calc_signed) = $function($a, $b, $c, $d);
            println!("\t= ({:x}, {:x}, {})", calc_result, calc_carry, calc_signed);
            assert!($result == calc_result);
            assert!($carry == calc_carry);
            assert!($signed == calc_signed);
        }
    )
}

func_test_tuple!(add_with_carry_1, add_with_carry,1,  0,       1,0);
func_test_tuple!(add_with_carry_2, add_with_carry,0,  u64::MAX,u64::MAX,0);
func_test_tuple!(add_with_carry_3, add_with_carry,1,  u64::MAX,0,1);
func_test_tuple!(add_with_carry_4, add_with_carry,10, u64::MAX,9,1);

func_test_tuple!(add_with_carry_5, add_with_carry, u64::MAX, u64::MAX,u64::MAX-1,1);

func_test_triple!(signed_add_with_carry_1, signed_add_with_carry, 0, false, 0, false, 0, 0, false);
func_test_triple!(signed_add_with_carry_2, signed_add_with_carry, 0, true, 0, false, 0, 0, false);
func_test_triple!(signed_add_with_carry_3, signed_add_with_carry, 1, true, 0, false, u64::MAX, 1, true);
func_test_triple!(signed_add_with_carry_4, signed_add_with_carry, 0, false, 1, true, u64::MAX, 1, true);
func_test_triple!(signed_add_with_carry_5, signed_add_with_carry, 0, false, 10, true, u64::MAX-9, 1, true);

func_test_triple!(signed_add_with_carry_6, signed_add_with_carry, u64::MAX, false, u64::MAX, false, u64::MAX-1, 1, false);
func_test_triple!(signed_add_with_carry_7, signed_add_with_carry, u64::MAX, true, u64::MAX, false, 0, 0, false);
func_test_triple!(signed_add_with_carry_8, signed_add_with_carry, u64::MAX, true, 0, false, u64::MAX, 0, true);
