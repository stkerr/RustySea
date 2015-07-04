extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test!(basic_rem_1, %,1,1,0);
op_test!(basic_rem_2, %,1,2,1);
op_test!(basic_rem_3, %,2,1,0);
op_test!(basic_rem_4, %,3,2,1);
op_test!(basic_rem_5, %,10,F,1); // 0x10 % 0xF
op_test!(basic_rem_6, %,1F,F,1);
op_test!(basic_rem_7, %,100,10,0); // 0x100 % 0x10 => 1024 % 16
op_test!(basic_rem_8, %,10000000000000000,1,0);
op_test!(basic_rem_9, %,10000000000000000,10,0);
op_test!(basic_rem_10, %,1000000000000000F,10,F);
op_test!(basic_rem_11, %,51000000000000030F,400,30F);

