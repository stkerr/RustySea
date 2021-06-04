extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test_eq!(basic_cmp_eq_1, "0x1" == "0x1" == true);
op_test_eq!(basic_cmp_eq_2, "0x1" != "0x2" == true);
op_test_eq!(basic_cmp_eq_3, "0x1" == "0x2" == false);
op_test_eq!(basic_cmp_eq_4, "0x1" != "0x1" == false);
op_test_eq!(basic_cmp_eq_5, "0x10000000000000000000000" != "0x10000000000000000000000" == false);
op_test_eq!(basic_cmp_eq_6, "0x10000000000000000000000" == "0x10000000000000000000000" == true);
op_test_eq!(basic_cmp_eq_7, "0x10000000000000000000000" == "0x10000000000000000000000" == true);

op_test_eq!(basic_cmp_lt_1, "0x1" <= "0x1" == true);
op_test_eq!(basic_cmp_lt_2, "0x1" < "0x1" == false);
op_test_eq!(basic_cmp_lt_3, "0x1" > "0x1" == false);
op_test_eq!(basic_cmp_lt_4, "0x1" >= "0x1" == true);
op_test_eq!(basic_cmp_lt_5, "0x6" > "0x1" == true);
op_test_eq!(basic_cmp_lt_6, "0x10000000000000000000000000000000" > "0x1" == true);
