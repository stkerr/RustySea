
extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test!(dynamic_test_0, "-0x136981e28f6db29bbf165ec5e79990d7e" - "-0x1e9d2837dcd8e9886dcfce02bf868fe90" == "0xb33a6554d6b36ecaeb96f3cd7ecff112");

op_test!(dynamic_test_1, "-0x2b62d68d3de1ae71b90713e575be2f9" * "0x3a37d88c964770fe49f1cb12a871f4ab" == "-0x9dddb8b50e9f3bd1adaefff5d110af7e3ebbc0572f2da7aa1e54ec869ef053");

op_test!(dynamic_test_2, "-0x1c515b726c7a6640f86151a7d8e8a22bf" + "-0x42954810b3666b82a6bb28f73a49ffcd" == "-0x207aaff377b0ccf922cd04374c8d4228c");

op_test!(dynamic_test_3, "0x3131bcb67c62110e7059b0044445d3f0" + "0x1f68ba9200caac1f9d1de3d759422b8ea" == "0x227bd65d6890cd3084237ed79d8688cda");

op_test!(dynamic_test_4, "0xad" & "-0xfd" == "0x01");

op_test!(dynamic_test_5, "-0x1e4986f98f69be13dad89ea218a448fc2" * "0x91a34c1cebeacdd6d10a703313ce6353" == "-0x113af7458b2755531ba2d0dbd179c629ce5b6cb787a03f9da07a3bed56beea1e6");

op_test!(dynamic_test_6, "-0x10411936124039c16a5997ee5fb753f68" * "0xb3ebf2672e6cac0c22edec9548e40d53" == "-0xb6c7fc70a5e84442ca5d9bcc42de05894706abab287280a924bc881f33dbd6b8");

op_test!(dynamic_test_7, "-0xa0dd6a93e42a980ce8788f1c07e5b0ab" * "-0x1f507cad3f0f1dcb7745d8eba5f2f0e73" == "0x13ad637153b91b1b7a082ee4d1da2d9b154c813c0a9d2253b820fb7ff003cb6d1");

op_test!(dynamic_test_8, "0x1d23baa14d087bb7c680801153e46f938" & "0xea218d7108a10cf7dbd5cb71afd5f10" == "0x22208141082104c680800151a445910");

op_test!(dynamic_test_9, "0x13e8b5b358691f26f43307267b46a0b1f" + "-0xec9e9f87cffb8de6b9a56a1738cc3557" == "0x51ecbbadb6966488898b08507b9dd5c8");
