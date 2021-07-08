
extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

op_test!(dynamic_test_0, "0x76d6717cb3932cfd" * "0xb184eaf18efd0027" == "0x5267f819e272917b22a501942774da8b");

op_test!(dynamic_test_1, "-0x11ecb27ad117aafc4" * "-0x14d165e0d27515c8f" == "0x17527516edd8ac0661fc6798153b69e7c");

op_test!(dynamic_test_2, "-0x1a07da4c69ceaf5f" * "-0x2747d7b1534bc679" == "0x3fe805dcf8c30fdf4c7540c0c295de7");

op_test!(dynamic_test_3, "0xd1977f9bd445b63" * "-0x59ce289df5761197" == "-0x49867486db014f8701a868f0c057a65");

op_test!(dynamic_test_4, "0x98ef97a4596e02fc" * "-0x7a22cb826e222e58" == "-0x48f6f7aa5f2801128590266d2fd24ea0");

op_test!(dynamic_test_5, "0x3b9e6ad825f3f374" * "-0x18ac193eba521fbbe" == "-0x5beee5520f9948baa6d8717a80b56c18");

op_test!(dynamic_test_6, "-0x1e09a67def868fbe9" * "0x5a257b2bf353a36a" == "-0xa93ca6121935f8fc20806ec88b68a97a");

op_test!(dynamic_test_7, "-0x1febe648d1068098d" * "0x1b4dcf411ed088797" == "-0x3679515cfe48db23aff1b8d505fcefd2b");

op_test!(dynamic_test_8, "-0xdf8083282258d58a" * "-0xcca5fe214b25b0aa" == "0xb2ab5437178b35e8a7a420742cbeada4");

op_test!(dynamic_test_9, "0x8db623a629722b9f" * "0x3c42836df17f3c32" == "0x215b821133078cdb0f4de2922d66c90e");

op_test!(dynamic_test_10, "0x15980f6c272ce8d4c" * "-0x174eb39ade641bc72" == "-0x1f74cddcd7d2e2baba2f8d0a6f80abbd8");

op_test!(dynamic_test_11, "0xd2109838b07203cc" * "-0x17e488f091daaa37a" == "-0x139b04d2e2730acb18bf7cfad5038b338");

op_test!(dynamic_test_12, "0xd46290203b8fa68f" * "0x4fdd0fa1dc57cf9b" == "0x4241d0886d70d4c2c2e82590b6407995");

op_test!(dynamic_test_13, "-0xf6f855d68a5e72b9" * "0x9a3c141f5b339338" == "-0x94cb5940d4486ef34c4c74621e645378");

op_test!(dynamic_test_14, "-0x1067acdba219fab5b" * "0x180586df66f31ddae" == "-0x18a12df8ce007611832f296d7f2df06da");

op_test!(dynamic_test_15, "-0x879f23a32720aaa9" * "-0x1c4ba12b89fa2627" == "0xefd78e76f5150f0ff214a47e95915bf");

op_test!(dynamic_test_16, "-0x5a60f6e970c437b2" * "-0xdbea0aa8b2ca2a7c" == "0x4da3939ebd3ee1caadbacff086a22e38");

op_test!(dynamic_test_17, "0x1690ae171a7a9e1bb" * "0x19e03f4af7fbf9057" == "0x247e52ccc52f4b30ffef4757fb939e68d");

op_test!(dynamic_test_18, "-0x6c127e7f83c848ee" * "-0x16f35c27e74d4f3b3" == "0x9b053548649b9528976fa14c7f5ce86a");

op_test!(dynamic_test_19, "0x19988f4df83a1cfe2" * "0x140b9b03b746b188c" == "0x201143ffd9f37e044be747e7d7370df98");

op_test!(dynamic_test_20, "0xeaee8c32cec2eccd" * "0x9730c3f1b594e570" == "0x8abf753dc77bbb2396a73749aa9efab0");

op_test!(dynamic_test_21, "0x1d5cc7c4a231a30c9" * "-0x5a1582f561ae3c7" == "-0xa5515dcc697eea11979c4b4fd08273f");

op_test!(dynamic_test_22, "0x1a370425c7705e053" * "0x733883ec544606b1" == "0xbcc80683b537bca334dfcb7301040b63");

op_test!(dynamic_test_23, "0x1b407032c1af4078a" * "-0x101a9e8bc0e14f054" == "-0x1b6dc6f37072f1f89c456708fc1ebd948");

op_test!(dynamic_test_24, "-0xbd2ed0bc85700812" * "0x91a3da7d72c6fc95" == "-0x6ba09a80420bc6677afd6e1c34126a7a");

op_test!(dynamic_test_25, "-0xd13cd5b9a924d9f5" * "-0x1da499622ed5ce685" == "0x183a6c8c8e66fe2f8e4673efc21035a49");

op_test!(dynamic_test_26, "0xda40b86fecfa2f4d" * "-0xb629ebc7b4ac7135" == "-0x9b4dc080c80d74eb8fde2bfb6a68c7f1");

op_test!(dynamic_test_27, "0x191e6baad3b62e8cb" * "-0x1ac9529d48e98bf7a" == "-0x2a0d7ed0fecf30ce28260a0d3eb5a65be");

op_test!(dynamic_test_28, "0xdcc67867bf82f6b3" * "-0xa6cd92b15cf1afbb" == "-0x8fd9fb83d74f74754d5ce268fdd191c1");

op_test!(dynamic_test_29, "-0x1f6bf06107ac54a8b" * "0x15fffc364296a9c55" == "-0x2b346314fd851adc28de97c361d7c7427");

op_test!(dynamic_test_30, "0x16c6434a9b48d85e9" * "0x1ac01cbcea78f927" == "0x2613a16898b6cfdef466e851e807077f");

op_test!(dynamic_test_31, "-0x1280ae35c9ff5b483" * "0x152100554b12aafd4" == "-0x186f0e708a7e0b585799184279e5d097c");

op_test!(dynamic_test_32, "-0x1d18f8f9651dbbc87" * "-0x85b2898552ff31dc" == "0xf32445833969334d16ba48407064db04");

op_test!(dynamic_test_33, "0x1b709e7647e4d6b70" * "-0xe72bc25454a549a5" == "-0x18c74fbbbad9ddf3f2236bbb07ab92f30");

op_test!(dynamic_test_34, "-0x1050c5a5a76a72c44" * "0xb801690a533fff04" == "-0xbba25119f811840c32e08ac050706d10");

op_test!(dynamic_test_35, "0x1ec0a5be063dc33a0" * "-0x12e85807fc992b9da" == "-0x24574d0bf3be391cf47c5d9bd3c129640");

op_test!(dynamic_test_36, "0xefbef62d82b6e0b1" * "-0x114881005709910c1" == "-0x102f94ddce0ab26fae6630589b3b37571");

op_test!(dynamic_test_37, "-0x154750cea623a9860" * "-0xa7939098ef7e5ee4" == "0xdedc9af08bef7a54c7af16e99f62f580");

op_test!(dynamic_test_38, "0xc23251b5a00ede96" * "-0x16c73d2bbce8dc594" == "-0x1147768a4b2a4c17b88a2c40b46801cb8");

op_test!(dynamic_test_39, "0x194fe9030043e9ed4" * "0x1e26b254b0785ec00" == "0x2fb30cce02961af72bbe55ae60a8f7000");

op_test!(dynamic_test_40, "-0x1ab34ef7aad81908e" * "-0x83f19bbc19768031" == "0xdc2f47442ba3b4cc37b6f05a7d87ab2e");

op_test!(dynamic_test_41, "0x1f5e81a1a15275378" * "-0x1e22c17db6325e19e" == "-0x3b15573cf9299611b89e87c4f16f9fc10");

op_test!(dynamic_test_42, "-0x187554a190fde43ff" * "0x162896e555c6540bc" == "-0x21df605c471c0a558ba3af72aafd4af44");

op_test!(dynamic_test_43, "0xcddd8b9f7d88a82c" * "-0x1198d52bfa24c1d9b" == "-0xe269d3d3092acf0079cdea9b41dacea4");

op_test!(dynamic_test_44, "0x47c6bfceb4eccc2c" * "-0x17ff10f57ccf6ea57" == "-0x6ba5ef5e15867fc47e88a5d331619af4");

op_test!(dynamic_test_45, "0x2356e57292d8f367" * "-0x1d9177f07d6ac9fe4" == "-0x414ecc487f74a0753392494c2b99c0bc");

op_test!(dynamic_test_46, "0x7ab22e87821333f" * "-0x2be58e08dab08a17" == "-0x1509f34776418b97f21dc1391eb90a9");

op_test!(dynamic_test_47, "0x1becf7edc96cb382" * "0x1669e96fa4af2d198" == "0x271eaf6fb34cc2e521afa43a9bfbb730");

op_test!(dynamic_test_48, "-0xe8243d148814f60a" * "-0x106c86d16cc6d23d1" == "0xee4ad5bed5ce9a77ea7db836b1023c2a");

op_test!(dynamic_test_49, "-0xdb751a9c54421a01" * "-0x595a219086dbaa25" == "0x4c98fa31b229540a7b521844d7ad6c25");

op_test!(dynamic_test_50, "-0x1d34b5dd8d1777bff" * "0xb4df8d98b5b52497" == "-0x14a290f1a76458ecd5e6a7ee65134ff69");

op_test!(dynamic_test_51, "-0x7caa21a41133e2a7" * "-0xa3c22ecd63f0b9dd" == "0x4fbee3238c3c831fbe7a4e176a25592b");

op_test!(dynamic_test_52, "-0xdc5c7a322b8aa6af" * "0x98e660e5548dc052" == "-0x839d370ba841ed524ff2f95429cfa40e");

op_test!(dynamic_test_53, "0x7693ff0c6faba181" * "-0xfd87ff872b1fd3bd" == "-0x756f4176d0fdfec00126902921728f3d");

op_test!(dynamic_test_54, "0x1cd51d4f2911167aa" * "0x119d0c92fb6f5839" == "0x1fbd70fc6823d1f26bdfda03e93884da");

op_test!(dynamic_test_55, "0x8f13a0eb80c54e5b" * "-0xf1705211a335d8eb" == "-0x86f040f24052cbbadd8e109dd912b589");

op_test!(dynamic_test_56, "-0x558525bc6c7776c6" * "-0x1f78add07ca2f2476" == "0xa83700c21c73b3f5e9653f2bd61e9744");

op_test!(dynamic_test_57, "-0x19bfb59db2ab954bd" * "-0x11d7a07c273ec51e0" == "0x1cb6b354e9f7ffec18a6efe6f0b35f260");

op_test!(dynamic_test_58, "-0xb534beffd46e2209" * "-0x502a37a8f121ae1a" == "0x38be5dbd32b012132e339d834a7a92ea");

op_test!(dynamic_test_59, "0x7466d5dc1afb0bed" * "-0x7641307d8e5b8360" == "-0x35c50abc205ce9afe632eeec487dbfe0");

op_test!(dynamic_test_60, "0x129013c743754d6f5" * "0xc6bc2beeea11936" == "0xe69144a2ac017f385f5f9f3eaf844ae");

op_test!(dynamic_test_61, "-0x8efae662cdb021a2" * "0x12146a4a7b145cf66" == "-0xa190b2a6ed7e211a58d7a0177209648c");

op_test!(dynamic_test_62, "-0x1cfe6b6587d539f3a" * "0x1543d316e04dd7ba4" == "-0x2688d4db980632b7aa9fd5980d424df28");

op_test!(dynamic_test_63, "-0xe2dcba46bf34621b" * "-0xa7d5f7a4f3d184ed" == "0x94bb9aa048cb0b345753a745bf1fbeff");

op_test!(dynamic_test_64, "-0xde36e6754e0dd8a1" * "0x3abdd62d30df6253" == "-0x32fd38a7d702878d8339e0e47fa9de33");

op_test!(dynamic_test_65, "-0xc58d605fb838a04" * "0x125b646db260bc5b8" == "-0xe2a7787dd78d77e263fed6885ec46e0");

op_test!(dynamic_test_66, "0xf4f2886d4fdbfab8" * "0xa073458158a99b58" == "0x9985e0b8bcd639c4d8ab5a0d67e39740");

op_test!(dynamic_test_67, "-0xf05c1b415d0d39a1" * "-0x1a66c39c8bd31ceae" == "0x18c9d720a3aa0ab2a4fd49129c82db96e");

op_test!(dynamic_test_68, "-0x2598774905673e27" * "0xd4c337d3bce5587d" == "-0x1f3eee1e2f2144705ee49aa75da9c10b");

op_test!(dynamic_test_69, "-0x1ad3032bcbfacce0c" * "-0xc222eeeca715249c" == "0x145791769560a8cb96b7c049fd5433f50");

op_test!(dynamic_test_70, "0x1cf3b9a145202d7ba" * "0x8724b6d0e00826b7" == "0xf48add70b77591b6a8b4b8ac89ddd1f6");

op_test!(dynamic_test_71, "0xe4472cda8dbe5a7" * "0x1977837e4e80fd8b2" == "0x16b585b83e6c19c29100f036bf73961e");

op_test!(dynamic_test_72, "-0x58a3fb9e9a6d7e6b" * "-0xf0ea565802d65b95" == "0x536adfae3fa115c131962d81511c9d47");

op_test!(dynamic_test_73, "0x31848f00b410efa" * "-0xc57f421f427e983a" == "-0x26339f98b4b285dda9ea13900add4a4");

op_test!(dynamic_test_74, "-0x1d15ec42b913a1731" * "-0xd12cd2febba7b028" == "0x17c3fda001720d19e6542fb1a8cfc4fa8");

op_test!(dynamic_test_75, "0x1be5bae6c8718405c" * "0x1153895af3c193627" == "0x1e35bdcc67be5a403e62ee25f8b413604");

op_test!(dynamic_test_76, "-0x793eecb6a4d49798" * "0x8bef4916f36ef214" == "-0x42467ee3fa3703dde2072464433987e0");

op_test!(dynamic_test_77, "-0x1c8fbc08bf5a4438f" * "-0x43f31e411cdb310c" == "0x794be05e222342e7ce9dd4cc63f689b4");

op_test!(dynamic_test_78, "-0x10a2970b479490a90" * "0x6aa45134b7c4e44b" == "-0x6edfffa4d9fa44d952d2889d9c0e5830");

op_test!(dynamic_test_79, "-0x17ea37a524f41cd63" * "0x4bd41286d010b689" == "-0x7156dfe8688b88c1b7dbdd34786b4bfb");

op_test!(dynamic_test_80, "0x1c42ce8a0ea9888dc" * "-0x7be1b02694182a35" == "-0xdad03660ca53483d80444de19ca86d8c");

op_test!(dynamic_test_81, "-0x13872e82f5135a862" * "0x1aebd3afe6e7c771b" == "-0x20db7f6d576cd7595d19a25512d665056");

op_test!(dynamic_test_82, "0x8a7569ba43408360" * "0x1e43e56c50a306231" == "0x105e7b341c5a3f22f4a87047ff3a3e560");

op_test!(dynamic_test_83, "0x177f34e2b8a8645ff" * "-0x272afb91abebcb5e" == "-0x39853044f02cf2644357bf47ece3e8a2");

op_test!(dynamic_test_84, "0x17cb75b6e6adba5bf" * "-0x3fa28256ddfe5164" == "-0x5ea2cd5e5456f70a192df71f93c02d9c");

op_test!(dynamic_test_85, "0x1f28088b54f41cb31" * "0x133062c8a2ff0640d" == "0x255dc29afd06e51ff87ff1a9136a6757d");

op_test!(dynamic_test_86, "0xafb94ef5fdf2e71b" * "-0xe6d9f15cab4c634" == "-0x9e7616b2002fa9e37043fbe4111d37c");

op_test!(dynamic_test_87, "0x595cc00a792ff1e2" * "0x15866204028d4e9b4" == "0x783848449476d84b45e121a97804c4e8");

op_test!(dynamic_test_88, "0x33d76b9beb87ec3" * "0x4d373f5bda1e8196" == "0xfa2fb7aa191bac0ab0abd2a0d48942");

op_test!(dynamic_test_89, "-0x613fc0abba52b5d9" * "-0x13c82afc98be62f94" == "0x783c56faf61b70a34f2157ea1b29f874");

op_test!(dynamic_test_90, "0x106c09660ef3bd448" * "0x14a1b45ee1e29813a" == "0x152d03fef1f2574a7c26700e8c90e6050");

op_test!(dynamic_test_91, "-0x132f4efcf12adb381" * "-0x1935135c6e77e3177" == "0x1e398f57e0f9607d1badb9512ae9821f7");

op_test!(dynamic_test_92, "-0xf55a7578596281aa" * "-0x12988ccd1fa54f0c3" == "0x11d290ea45c04526459df2934ff60247e");

op_test!(dynamic_test_93, "0x20c49d7cbf890198" * "0x12bc6365315c22090" == "0x265f02f69df182c259d925598a73e580");

op_test!(dynamic_test_94, "-0x1c0e4c119bcdef5a9" * "-0xb977c5fd6cc7bfb1" == "0x14537550dc5de535c8ede6caf31cff0d9");

op_test!(dynamic_test_95, "0x14b31daf1a1014be2" * "0x1530f9afba87ddfc4" == "0x1b6a7355b5defbcd7d5b1ee43c371f708");

op_test!(dynamic_test_96, "0x77f8dbb8542f91c3" * "-0x1b85828af4153effa" == "-0xce5d0a415c1f38f9c69e245664c2656e");

op_test!(dynamic_test_97, "-0x182caedea049f5efd" * "0x5aeea4f683eaa6a8" == "-0x8963e98bebeb3e6f6eed02f8aa706408");

op_test!(dynamic_test_98, "-0x1a931c7aac666ce8" * "0x1d0342b8f61430cc2" == "-0x30300e05b972e14c18e110b5db7167d0");

op_test!(dynamic_test_99, "0x1bd4699f66f642e1c" * "0x401f0b165f452b09" == "0x6f87a5533314dca3ec152be48fd052fc");
