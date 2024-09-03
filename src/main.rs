use std::{
    collections::BTreeSet,
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use uplc::ast::{DeBruijn, Program};

fn main() {
    let args: Vec<String> = env::args().collect();

    let aiken_markers: BTreeSet<&str> = BTreeSet::from([
        "delay[(error)(force(error))]",
        "List/Tuple/Constrcontainsmoreitemsthanexpected",
        "ExpectednoitemsforList",
        "ExpectednofieldsforConstr",
        "ExpectedonincorrectBooleanvariant",
        "ExpectedonincorrectConstrvariant",
        "Constrindexdidn'tmatchatypevariant",
        "(force(builtinmkCons))])(force(builtinheadList))])(force(builtintailList))",
    ]);

    let plutarch_markers: BTreeSet<&str> = BTreeSet::from([
        "constring\"can'tgetanycontinuingoutputs",
        "constring\"PatternmatchingfailureinQualifiedDosyntax",
        "constring\"reachedendofsumwhilestill",
        "constring\"PatternmatchingfailureinTermCont",
        "constring\"ptryPositive",
        "constring\"pfromJust",
        "constring\"pelemAt",
        "constring\"ptryFrom(TxId)",
        "constring\"ptryFrom(POSIXTime)",
        "constring\"ptryFrom(TokenName)",
        "constring\"ptryFrom(CurrencySymbol)",
        "constring\"ptryFrom(PDataRecord[])",
        "constring\"ptryFrom(PScriptHash)",
        "constring\"ptryFrom(PPubKeyHash)",
        "constring\"ptryFrom(PRational)",
        "constring\"unsorted map",
        "(force(builtinheadList))])(force(builtintailList))])(lami_0[i_2[(builtinunConstrData)i_1]])])(force(force(builtinsndPair)))",
        "(force(builtintailList))])(force(builtinheadList))])(lami_0[i_2[(builtinunConstrData)i_1]])])(force(force(builtinsndPair)))",
        "Patternmatchfailurein",
        "Plutarch",
    ]);

    let plutarch_validators: BTreeSet<&str> = BTreeSet::from([
        "02652a93b8327ba64b6c0bb8dfb11a76cbb333a3fbb4243ddc0859ac",
        "038e9126b0113b9c496c245b256abfc2962161d8ed2fe119564a3933",
        "071bd7f4b5e059ea90e763467cf559167b21c82ef1cb5fe34fb7a9e5",
        "090daf80c177304234d4bf6248a95d1da6c880a712a83a40a456dde7",
        "0e82eb5f90eec1a9ed3ecc5d0c639de82a03ca6e3417bc0d0d01637e",
        "0fd1b854729d2e29290d47fe2173319aae3c4c885a955dc203f72759",
        "100b7e144b06bfd19c6d6478886cf6e22eca6c4f80ff4d1dc2c38d4e",
        "103231ad9f190abec0d7cb7cba383cea4278eb4cbb737bbc3e9bd8bf",
        "1313058e934ea72e7d31d6e3f296d0e844ddf0177dab7b11244280cb",
        "14386d24c827f3ce74bdf6c875e548d928e12036c145ebec14f04df5",
        "1579f32be90cf3825e7194568240dae5e0b87c7c40108ed7cdfe89f8",
        "187314a316d8584705690ff0e8f6d0e5fe0af7c2a1d0b997579bb45c",
        "19b9908183fc7180d82cb5e15bf9434f4eb644019b63dfd4ae4dbb0b",
        "19e609260d10c8b7c01024dd8e5c51cc3d1fda824eccac19e9f45d69",
        "1a9cf06cd17107f521441065eb64567f49451c83f8fed5cd6c6bdf0d",
        "1d1e863afcf2182c32eb5c83e11e8ffecdcb95216b6da7ed547f6149",
        "1e417fb7b39650586f5bcd5b1fce5afc3b0fd2d8feefa61866b39f7d",
        "21bd8c2e0df2fbe92137f78dbaba48f62308e52303049f0d628b6c4c",
        "21f07a4f016c05332a390494328968ab3127a4d638150f51a8874134",
        "26aea7e03a53e374c1d82c560a91b4147b8954d2ec4ecbd128e0c18c",
        "28dbd999eca72c7de7cbb75452fb613c56723da89e3591b821a0b52e",
        "2b48af6fd01c49d05ddee3badaba999c4cf8c41ca69c71928a3a6986",
        "301d6ffb386dc89d2257968d6c8503f40a1cf81f12e778aa83b6a3f8",
        "30ebb5f76e7fb684fee6e2cf4b37258d676682b2aa213eb3c49b3880",
        "3592cb9699d2137ad980c3be8613d89f84035f9c2dc4a5e6c3e5c9a1",
        "359ae06da4dbdd92c63743803f6be1bf1d24477645cddea07a5efb1b",
        "384d2b4b6279a10e1a3bc7168295ec98bbf43de17949065286f27597",
        "39fec77302f6a898a83df189926621566b246ea4686082a6ba830566",
        "3eca957ef02d32f3ae24fe7a8a0b210522de7abe32282203eb32fe0a",
        "3f13b8c57ac5b64920f511507662ed3a8ae7ef27210b37e6be149b67",
        "43e348ee331014ca148aba79ef99d1452f46c7d56a2cc3ccda3d8f2a",
        "440815e8750032eb272e9edb6194c45cd5268a8b62377aaaea05f4cd",
        "451e7db46cf0e46aacc15257b0af84510fe9da5bd7e28e4d37f31e8d",
        "452650490499ff68be41287e08662039deb04e723d0f33c8591cde7a",
        "452955e5578a59e97bd6e62071015be742cc243eac9614edff5851d9",
        "469772d2f93d70f92a4930fa608457392e58e480babf723ade7f9857",
        "4811377ea9279acb8fff1d65a225e19c0eca68bb5c1d836a47584e67",
        "48cd6201316d65ce895d6d9f255881a27986e71aba4d0af015825f19",
        "4986257bffd6bc8bda0e56fd7796c5d125fe496deb3a00c5196068a3",
        "503bf265b929d8aa4ea18d3307ac05a79ee209e5ea7fa25979fabc64",
        "5112947b8a6b78c4acccc915d7652bcf467f07dd65e312576c86814a",
        "51936f3c98a04b6609aa9b5c832ba1182cf43a58e534fcc05db09d69",
        "5818c3a124b3685c5a1bb916b79c0ef23fd0a0c6076ec588a51a4ab8",
        "5917ad9f27bb40482eac40027f9dd69898215ef4843a525c4355193b",
        "5bf90fe45e0f130a36947ff5ef69b162c0e19c88284ef95e830ef080",
        "5d211d7259fc8fb6eae43875d369cd676dfe680c5fd89fd73c0b5491",
        "5d79bdf7c3106bdcc4ee3f9710d0c395b616e4c89c6ea864e6eba708",
        "622338cdf5a7f0925a1218683acbefdd4dcfcef09bacada7e7482d30",
        "635287c717cb4b542d92e80affc3c84dafeb3cfdee3003e018f52c9b",
        "63d65418f841e9901d5ee634cc07377cca2be7fb2b797e3f0e0d8334",
        "648520089b3d89f5aa94adc0ad0b911f97677c8700925c4be6290f5b",
        "652b378a21deeb568dc6d338d8b26fd9c7c134d69d1ba713408573bb",
        "6876fa49e702ea22778506a22bc69cfe2477d108998a09ae83cd7816",
        "69da1f447abecc8493fb629c23c5df4ed9c8841aaf86b46b62991ed7",
        "6a0e34af4d51828a50c6f3d4b1610381de87f95d199a143efdcb8123",
        "6a836e8bb408ef5110a7477039a9f5fc41e55be9fc7eb5464def826b",
        "6bb0851fa2bf40c6729c9d5df97a4201fdea2b3ae169ad5242bfe47a",
        "71391f18fb131f28a230fa7f3b6c6099e447602b2bd2df5d046c5e99",
        "7a9946acc68e06efd7cf05ecc3633c25ed383dad17e6de3d41cc10fd",
        "7bd80451bde142f4e9d9af436de303fb4c0aaf51d96b14a41e2e07de",
        "7bf8d17b4ea7a5abb5ed56f70cea83f2c6055180dd5dbae86a458595",
        "7c0c2f947629a136f48f3c985e1ae878b85776408b244a74ddfb811d",
        "7d1c21644ce422011b4b0c7b7129875fb79bf31e25ddfd24eb1b355d",
        "7edda41b7048987d3524ae7fc447feeaa51f9296d6773692c7e94824",
        "7fc0d659f74debe817faa6487f0cbc7d8cec93997adaad543fbfff98",
        "80a185e239a191ecd715c93cebefdf0babcf8b9b051dd861936975df",
        "84ba5f03abefa84a86c32c3c4203bb5f5593536776e1abc905f2eff9",
        "854c7b463dca50b52be7448e586387200557a3feae51676aa855e77a",
        "866ec79823a71f803617c9cc76cfefbbd65b8dbd4a902dfa261f483a",
        "89ea1358d8147feb5ef1875fbad449f65719ced6102ce1600bf093a7",
        "8c126fd29c7a6d213b4ea46568fe60d4f405f8364105506ad47d2350",
        "8c5a532c6332f85e6a927750e129aba954eec6ead209cd72d08f5a32",
        "8d258b9d08dcab73f3165a11751d464b46056264091c1789da588726",
        "8e941c939dd9d78715db1d88c9b7f4b18d9af12b288e3f8f84314525",
        "932d2c4132dfdc1673ef6f1cce7a2741e0ed86f9f854cd76fbd1b029",
        "9392bfc154f465f6788192c0176feca44c4b43318585b246712629db",
        "948711b192d0703ff4a1bc0d565e681f64b3027b0cae562765b0c584",
        "959d9b08113e0edbc48ce2c23bb293e4e47590bf574fa4f811fb736b",
        "9885c2bc619cb1e09d63c59eebef3c27e7b796cdca3ff89fdbfb4fa1",
        "989f82644660e5cfc77889cbab443ad0729171973fbf8da39aaafdb5",
        "995f2396433ac81ec0770632e3d78971d0d9d6669921df5bb84cc86f",
        "9b0f13213e40bcf25850871a7cf8cfb359dd1f954b49f90cda3a5f16",
        "9ce90fdadd7089ebda384bf921521a1e7cb2e365dee681e0d3e2af0c",
        "9d7fbb38fe90a58ebac1c91f37c008bd6acedab4fae76f2201072df9",
        "a133763023c1f6ffb588daef0e56a062e15816164a44985c2613619b",
        "a3e56ea9d2db008038ce6fb32e500faef1523dcb042e5a637d633fc8",
        "a50ecf6e4d5621d034f0a09714da50cea8e94fcd77388954654d0479",
        "a54bc78ce92e48ca736034bebeae9dc1565d639b6225f0f5fb6cc297",
        "a904455f964515aa9290ca443efe93fe8d654c1ebc22b1562af07c39",
        "acee18e799d5b4e30046e2eaad1f75fc95997dd6ec37995f02670ccd",
        "ad334d2f48985b27d52fb074f28df88ff96ca73804a3dca38f7652b7",
        "add1efe119a861bd883aaa8616437187d4b215425b8bbc8afdcd5610",
        "b10144c36699ba48b21325dc5a08da6ce09496cfa6f7ef796eac6490",
        "b2effcce85662fbd52ea6517fcae417a9cd85db752f0f94c1c91a266",
        "b575da974ec756ba2afdccdc07cbd1839acaf0a24c4f80afd81d3fb6",
        "b61670381f47934a9f0aac56ba067733ccabe718b1dd8d6d0b469032",
        "bbbc2587e04ec1a574d6fc287ea57ef279dd9e970bebc36193ad50d2",
        "bc3de5c5af5513959c205dbefbdcf81505a4c59d16ec2471c5e856c0",
        "c0588460f21b1474de8f7621b3c90572ce05ee08e22b006bb2f21f64",
        "c467e4a9fa45ce830819645f72db86c7e46e647f9cf46b3b9671284a",
        "c63f80127de4782c341171f9b2a3da087d99504e815718ca03943b45",
        "c7958044e73c1537a8258af20aceb0ee3c7f0e832e900d30cc32695d",
        "c79b4145d351812445def14c3dddd04e5591e78020a7e976e805952b",
        "ca925263da4c59bf675f3cc485c894575cfe9b2b50ae5c3a360a62cd",
        "cee7be304ce9501a702fdc45cd42c724b60b99fd462a392e4b1a4553",
        "cffa9d6b81e11f9528346c55a676442098d6f012c10152e1a9d37e05",
        "d062d56e91347e47d9e8d7587722e394bded511a693cd0b09d0a8e16",
        "d1967e6d23962150fb25168829002049feb9133740fa738ed4ca5bd7",
        "d3eb5055c7606be1828b134bfe6bf26e85d14f2ae97dc741685bc668",
        "d63f64eb6daeb633fe63ec17818b563a806de8940a7e41c8e7c513c0",
        "d9a67663ab06151dab3a314311b115c992f1500e38a19ea1ec32b928",
        "da11a50969a7f77225a9e9e2c86e43d391a69dd47f339a4fd830d165",
        "e88f72f6b4673db78efe148b1cd90c689115ddb3d7e356a0003148cf",
        "eb1c62309586ed7282eb37c4ff72cd30f0cf55349b781e26df16e101",
        "edc97eac064185d20abf8cf21d2260566adeda888bc70bb68bb8201e",
        "ee6b9483c3ab7dc6a0b56ac2d5e9c6227bb4d14124d1ebc5aaa89260",
        "ef8251dfa08c465bf9dd3ff714fa15b625c35a46fb3ebc5f0f7405fb",
        "f0fe71a68a750305915a43ac8efffa5f65e4b54d0bf1bcc45ca7b3e3",
        "f2538e1341feef79c6314d571118a95a988a433a7e0dd70ba5cddb18",
        "f3862fbe79aeca777b35fe3fa34e600a25caf27ec230f33117043f1b",
        "f42059f8bafc3cceec94eb5d09c656b5685ef179b13b55baeec765e2",
        "f42f7a0c16e377a7313404e8880386a6f8e5725db39985d25757b6ba",
        "f61e5cf96a45049adc9fb4092bb2909756c9fef7f501ca51a02b99ce",
        "f6578308a023f60013a5dc85651c489bcdec71326f74ad0fe5bae69c",
        "f73843062a63e9b882bfda8dd87360c588ce664510c22fd52bb49c4a",
        "f7c838d95288aaa94fbf59211eebab72805e5980aa23c3bfb19ecff9",
        "f8c746bfbd35add98ab0079c5f55642060007c36a828a61fd2cdc903",
        "fa0a70c3944ffefda77daf6e0660f07ef477f07a705fa162cd36f7c0",
        "fa3603d2283e3dadee0b5810faf590ea9da6c7fea91095657f98a9c2",
        "fae6df1f28636c19f20033440e2daad8da929679ca3a5e9cda6b6dbe",
        "fda79c6aa9bc492a7bf1055cad9dd55c1ff0df30e0ef5202d429e24d",
        "fe4ee3dc2ebe6d57cffda8f896ce93c56d094e26607d6823b1ddd3e8",
        "d5910c330ed47f23f0cc86132332bd081f193220e302f6c180acb3a2",
        "c6d53bf97c0d7e38368b2ed6b2dcf8d5fb4df0ac533fa22458e04f45",
        "3b07e0f2a262fa1436df3c91e420d57ad9fd46aa04377ec80a05ee3f",
        "c51806ac8fd893646c12fbd8483abdd4db5b587e16e155a1bc1b289f",
        "4c820aadf6ae8c755430455f5803d283bde0b20114bd93a8f381c72c",
        "40243894c103511d74c884e8ed231c0ca7df00644bd4afe2b82044fb",
        "59c9aa4ea305356d37e5948ef2a3e6477674c80cff384889fdff902b",
        "c677cdb77d115df5d3772aaad91ea9e4b5d4a7f970308183dd193f1c",
        "ba3501cd170c96349c342c5ef4242c596b58afaecdcffc0bb04af0ec",
        "30e02ff8576298babd301cee58928d7b320364abeae556c46e3cf42d",
        "d5910c330ed47f23f0cc86132332bd081f193220e302f6c180acb3a2",
        "0397b7b0f2fa064645785c2131fa7dd0a3a160ce603a3a30bff2df49",
        "d612be7ab0bdbd3d728b922e422da843de33ee71bd13c19e78b32080",
        "5ae1a14e032b8583037304500163b72fb638a507447cbc00c6e6e2da",
        "18c91bdff54ad8f4d3618818f36b99e401caa7eab153b42f51311cb0",
    ]);

    let opshin_markers: BTreeSet<&str> = BTreeSet::from([
        "constring\"KeyError\"",
        "constring\"NameError:",
        "constring\"ValueError:datumintegritycheckfailed\"",
    ]);

    let helios_markers: BTreeSet<&str> = BTreeSet::from([
        "validationreturnedfalse",
        "force(builtinifThenElse))[[(builtinequalsInteger)[(force(force(builtinfstPair)))[(builtinunConstrData)",
    ]);

    let plutus_tx_markers: BTreeSet<&str> = BTreeSet::from([
        "constring\"L0\"",
        "constring\"L1\"",
        "constring\"L2\"",
        "constring\"L3\"",
        "constring\"L4\"",
        "constring\"L5\"",
        "constring\"L6\"",
        "constring\"L7\"",
        "constring\"L8\"",
        "constring\"L9\"",
        "constring\"La\"",
        "constring\"Lb\"",
        "constring\"Lc\"",
        "constring\"Ld\"",
        "constring\"Le\"",
        "constring\"Lf\"",
        "constring\"Lg\"",
        "constring\"Lh\"",
        "constring\"Li\"",
        "constring\"PT1\"",
        "constring\"PT2\"",
        "constring\"PT3\"",
        "constring\"PT4\"",
        "constring\"PT5\"",
        "constring\"PT6\"",
        "constring\"PT7\"",
        "constring\"PT8\"",
        "constring\"PT9\"",
        "constring\"PT10\"",
        "constring\"PT11\"",
        "constring\"PT12\"",
        "constring\"PT13\"",
        "constring\"PT14\"",
        "constring\"PT15\"",
        "constring\"PT16\"",
        "constring\"PT17\"",
        "constring\"PT18\"",
        "constring\"Pa\"",
        "constring\"Pb\"",
        "constring\"Pc\"",
        "constring\"Pd\"",
        "constring\"Pe\"",
        "constring\"Pf\"",
        "constring\"Pg\"",
        "constring\"S0\"",
        "constring\"S1\"",
        "constring\"S2\"",
        "constring\"S3\"",
        "constring\"S4\"",
        "constring\"S5\"",
        "constring\"S6\"",
        "constring\"S7\"",
        "constring\"S8\"",
        "constring\"C0\"",
        "constring\"C1\"",
        "41786f4f7261636c655631",
        "41756374696f6e457363726f7",
    ]);

    let plutus_tx_validators: BTreeSet<&str> = BTreeSet::from([
        "15b95fdaceeb507073a1bd198803373beeafbd82560fbf8abe9073ff",
        "ea184d0a7e640c4b5daa3f2cef851e75477729c2fd89f6ffbed7874c",
        "a65ca58a4e9c755fa830173d2a5caed458ac0c73f97db7faae2e7e3b",
        "00fb107bfbd51b3a5638867d3688e986ba38ff34fb738f5bd42b20d5",
        "1b3c5a646a018e0cfbd40fba97518c8e955e5869f0afd4f8c568493e",
        "c2afd87ff836f64a20c33dc252e850cdd55f31627d64012d0960856f",
        "785331daeba8b0f238183d502e14ace037d1167ececb99b744a2aaaf",
        "bb69ba38801dec818fdc94d6e24c3f3866fee298a1bea57b56600e8a",
        "cae07bd684bcb2e2f15356882deac1b6cc782b025dc23aae10758471",
        "0454355b392202e594ce74be2f5ff9ae64c6ac0fbb45cb676f8078a2",
        "f33bf12af1c23d660e29ebb0d3206b0bfc56ffd87ffafe2d36c42a45",
        "6e3af9667763e915c9b3a901d3092625d515c2ad6d575eac92582aa8",
        "909133088303c49f3a30f1cc8ed553a73857a29779f6c6561cd8093f",
        "7a8041a0693e6605d010d5185b034d55c79eaf7ef878aae3bdcdbf67",
        "de9b756719341e79785aa13c164e7fe68c189ed04d61c9876b2fe53f",
        "ffcdbb9155da0602280c04d8b36efde35e3416567f9241aff0955269",
        "af3d70acf4bd5b3abb319a7d75c89fb3e56eafcdd46b2e9b57a2557f",
        "8c9a2d459d2d8dc7c11192f971ab647fac65833121b7e8181e583c64",
        "fc8fbdf64f25e04ec78052aa1a997b93867608951608fd74ca2fa83b",
        "cae07bd684bcb2e2f15356882deac1b6cc782b025dc23aae10758471",
        "c2afd87ff836f64a20c33dc252e850cdd55f31627d64012d0960856f",
        "73ede893f547edbd25da6953fda33caacd01f44047922bf7c5ceb951",
        "1b3c5a646a018e0cfbd40fba97518c8e955e5869f0afd4f8c568493e",
        "785331daeba8b0f238183d502e14ace037d1167ececb99b744a2aaaf",
        "15b95fdaceeb507073a1bd198803373beeafbd82560fbf8abe9073ff",
        "ea184d0a7e640c4b5daa3f2cef851e75477729c2fd89f6ffbed7874c",
        "f1ec90d33208778214cdc7fa90858ac5620253d99f84c10335928cab",
        "fea3f75281aa7f3f9f1a488030006cecd96290e9d3e7921a6b10c903",
        "00fb107bfbd51b3a5638867d3688e986ba38ff34fb738f5bd42b20d5",
        "e8baad9288dc9abdc099b46f2ac006b1a82c7df4996e067f00c04e8d",
        "7045237d1eb0199c84dffe58fe6df7dc5d255eb4d418e4146d5721f8",
    ]);

    let pluts_validators: BTreeSet<&str> = BTreeSet::from([
        "4ab17afc9a19a4f06b6fe229f9501e727d3968bff03acb1a8f86acf5",
        "28bbd1f7aebb3bc59e13597f333aeefb8f5ab78eda962de1d605b388",
        "ab658d65b5717bf07bd3b1a9ad28d31c183811bba4076aeace9feb8e",
        "0c70d8047139103546f0e76aafecfdf0667cbb397c8976f40ae8fcb3",
        "cd2333712262ca401b81605f6ae44a58fdb656f0af32d00598d716e3",
        "8faf20cd4d98e4d5fe8a1e0c966a7c49e103610d582cc1e1cc5b1e35",
    ]);

    let mut is_first_validator = true;

    if let Ok(lines) = read_lines(&args[1]) {
        for row in lines.into_iter().flatten() {
            let hash: String = row.chars().take(56).collect();
            let cbor: String = row.chars().skip(57).collect();

            let program = Program::<DeBruijn>::from_hex(&cbor, &mut Vec::new(), &mut Vec::new())
                .unwrap()
                .to_pretty()
                .replace(['\n', ' '], "");

            let delim = if is_first_validator { "[" } else { "," };

            if hash == "6027a8010c555a4dd6b08882b899f4b3167c6e4524047132202dd984" {
                println!("{delim}[\"{hash}\",\"marlowe\"]");
                is_first_validator = false;
                continue;
            }

            if plutus_tx_validators.contains(&hash.as_str()) {
                println!("{delim}[\"{hash}\",\"plutus-tx\"]");
                is_first_validator = false;
                continue;
            }

            if plutarch_validators.contains(&hash.as_str()) {
                println!("{delim}[\"{hash}\",\"plutarch\"]");
                is_first_validator = false;
                continue;
            }

            if pluts_validators.contains(&hash.as_str()) {
                println!("{delim}[\"{hash}\",\"plu-ts\"]");
                is_first_validator = false;
                continue;
            }

            if any_marker(&program, &plutus_tx_markers) {
                println!("{delim}[\"{hash}\",\"plutus-tx\"]");
                is_first_validator = false;
                continue;
            }

            if any_marker(&program, &aiken_markers) {
                println!("{delim}[\"{hash}\",\"aiken\"]");
                is_first_validator = false;
                continue;
            }

            if any_marker(&program, &helios_markers) {
                println!("{delim}[\"{hash}\",\"helios\"]");
                is_first_validator = false;
                continue;
            }

            if any_marker(&program, &opshin_markers) {
                println!("{delim}[\"{hash}\",\"opshin\"]");
                is_first_validator = false;
                continue;
            }

            if any_marker(&program, &plutarch_markers) {
                println!("{delim}[\"{hash}\",\"plutarch\"]");
                is_first_validator = false;
                continue;
            }

            println!("{delim}[\"{hash}\",null]");
            is_first_validator = false;
        }
    }

    println!("]");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn any_marker(program: &str, markers: &BTreeSet<&str>) -> bool {
    for marker in markers {
        if program.contains(marker) {
            return true;
        }
    }
    false
}
