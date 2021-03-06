extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test!(basic_rem_1, "0x1" % "0x1" == "0x0");
op_test!(basic_rem_2, "0x1" % "0x2" == "0x1");
op_test!(basic_rem_3, "0x2" % "0x1" == "0x0");
op_test!(basic_rem_4, "0x3" % "0x2" == "0x1");
op_test!(basic_rem_5, "0x10" % "0xF" == "0x1"); // 0x10 % 0xF
op_test!(basic_rem_6, "0x1F" % "0xF" == "0x1");
op_test!(basic_rem_7, "0x100" % "0x10" == "0x0"); // 0x100 % 0x10 => 1024 % 16
op_test!(basic_rem_8, "0x10000000000000000" % "0x1" == "0x0");
op_test!(basic_rem_9, "0x10000000000000000" % "0x10" == "0x0");
op_test!(basic_rem_10, "0x1000000000000000F" % "0x10" == "0xF");
op_test!(basic_rem_11, "0x51000000000000030F" % "0x400" == "0x30F");
