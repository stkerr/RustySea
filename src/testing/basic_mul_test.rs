extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test!(basic_mul_1, "0x1" * "0x1" == "0x1");
op_test!(basic_mul_3, "0x1" * "0x2" == "0x2");
op_test!(basic_mul_2, "0x1" * "0x0" == "0x0");
op_test!(basic_mul_4, "0x2" * "0x1" == "0x2");
op_test!(basic_mul_5, "0x0" * "0x1" == "0x0");
op_test!(basic_mul_6, "-0x1" * "0x0" == "0x0");
op_test!(basic_mul_7, "-0x1" * "0x2" == "-0x2");
op_test!(basic_mul_8, "0x2" * "0x2" == "0x4");
op_test!(basic_mul_9, "0x400" * "0x400" == "0x100000");
op_test!(basic_mul_10, "0x10000000000000000" * "0x2" == "0x20000000000000000");
op_test!(basic_mul_11, "0x10000000000000000" * "0x10" == "0x100000000000000000");
op_test!(basic_mul_12, "0x10000000000000000" * "0x10000000000000000" == "0x100000000000000000000000000000000");
op_test!(basic_mul_13, "0x10000000000000001" * "0x1" == "0x10000000000000001");
op_test!(basic_mul_14, "-0x10000000000000001" * "0x1" == "-0x10000000000000001");
