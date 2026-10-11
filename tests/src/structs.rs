use flat_message::*;

#[test]
fn check_simple_serde() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
    assert_eq!(MyDataV1::DATA_FORMAT, DataFormat::Struct4);
}

#[test]
fn check_simple_serde_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header for Test structure
            146, 207, 36, 196, // Hash for MyDataV1
            16, 44, 0, 0, // MyDataV1 - > 16 >> 2 = 4 fields, size = 44 octeti
            5, 72, 101, 108, 108, 111, // Test::d::d -> Hello (size 5)
            2,   // Test::d::a
            4, 0, // Test::d::c
            3, 0, 0, 0, // Test::d::b
            0, 0, 0, // Padding
            14, 36, 12, 225, // Hash for Test::d::d (14 = string)
            1, 41, 12, 228, // Hash for Test::d::a (1 = u8)
            2, 44, 12, 230, // Hash for Test::d::c (2 = u16)
            3, 45, 12, 231, // Hash for Test::d::b (3 = u32)
            8,   // Offset of Test::d::d
            14,  // Offset of Test::d::a
            15,  // Offset of Test::d::c
            17,  // Offset of Test::d::b
            5,   // Test::a
            1,   // Test::x
            0, 0, // Padding
            32, 36, 12, 225, // Hash for Test::d (32 = Struct4)
            1, 41, 12, 228, // Hash for Test::a (1 = u8)
            1, 80, 12, 253, // Hash for Test::x (1 = u8)
            8,   // Offset of Test::d
            52,  // Offset of Test::a
            53   // // Offset of Test::x
        ]
    );
}

#[test]
fn check_simple_serde_align_8() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        e: Vec<u64>,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 8, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
            e: vec![1, 2, 3, 4, 5],
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
    assert_eq!(MyDataV1::DATA_FORMAT, DataFormat::Struct8);
}

#[test]
fn check_simple_serde_align_8_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        e: Vec<u64>,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 8, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
            e: vec![1, 2, 3, 4, 5],
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header for Test structure
            146, 207, 36, 196, // Hash for MyDataV1
            20, 97, 0, 0, // MyDataV1 - > 20 >> 2 = 6 fields, size = 97 octeti
            5, 0, 0, 0, 0, 0, 0, 0, // numver of elements from MyDataV1::e
            1, 0, 0, 0, 0, 0, 0, 0, // MyDataV1::e[0]
            2, 0, 0, 0, 0, 0, 0, 0, // MyDataV1::e[1]
            3, 0, 0, 0, 0, 0, 0, 0, // MyDataV1::e[2]
            4, 0, 0, 0, 0, 0, 0, 0, // MyDataV1::e[3]
            5, 0, 0, 0, 0, 0, 0, 0, // MyDataV1::e[4]
            5, 72, 101, 108, 108, 111, // Test::d::d -> Hello (size 5)
            2,   // Test::d::a
            4, 0, // Test::d::c
            3, 0, 0, 0, // Test::d::b
            0, 0, 0, // padding
            132, 34, 12, 224, // Hash for MyDataV1::e
            14, 36, 12, 225, // Hash for MyDataV1::d
            1, 41, 12, 228, // Hash for MyDataV1::a
            2, 44, 12, 230, // Hash for MyDataV1::c
            3, 45, 12, 231, // Hash for MyDataV1::b
            8,   // offset of MyDataV1::e
            56,  // offset of MyDataV1::d
            62,  // offset of MyDataV1::a
            63,  // offset of MyDataV1::c
            65,  // offset of MyDataV1::b
            5,   // Test::a
            1,   // Test::x
            0,   // Padding
            33, 36, 12, 225, // Hash for Test::d (33 = Struct8)
            1, 41, 12, 228, // Hash for Test::a (1 = u8)
            1, 80, 12, 253, // Hash for Test::x (1 = u8)
            8,   // Offset of Test::d
            105, // Offset of Test::a
            106  // Offset of Test::x
        ]
    );
}

#[test]
fn check_simple_serde_align_16() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        e: Vec<u128>,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 16, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            e: vec![1, 2, 3],
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
    assert_eq!(MyDataV1::DATA_FORMAT, DataFormat::Struct16);
}

#[test]
fn check_simple_serde_align_16_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        e: Vec<u128>,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 16, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            e: vec![1, 2, 3],
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header for Test structure
            0, 0, 0, 0, 0, 0, 0, 0, // alignament for d (16 bytes)
            146, 207, 36, 196, // Hash for MyDataV1
            8, 94, 0, 0, // MyDataV1 - > 8 >> 2 = 2 fields, size = 94 octeti
            0, 0, 0, 0, 0, 0, 0, 0, // alignament for e (16 bytes)
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // number of elements in e
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // e[0]
            2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // e[1]
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // e[2]
            2, // Test::a
            0, 0, 0, // Padding
            133, 34, 12, 224, // Hash for MyDataV1::e
            1, 41, 12, 228, // Hash for MyDataV1::a
            16,  // Offset of MyDataV1::e
            80,  // Offset of MyDataV1::a
            5,   // Test::a
            1,   // Test::x
            34, 36, 12, 225, // Hash for Test::d
            1, 41, 12, 228, // Hash for Test::a
            1, 80, 12, 253, 16,  // Offset of Test::d
            110, // Offset of Test::a
            111  // Offset of Test::x
        ]
    );
}

#[test]
fn check_simple_serde_timestamp() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        t: Timestamp,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            t: Timestamp::with_value(0x8899AABBCCDDEEFF),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t.x, t2.x);
    assert_eq!(t.a, t2.a);
    assert_eq!(t.d.a, t2.d.a);
    assert_eq!(t.d.b, t2.d.b);
    assert_eq!(t2.d.t.value(), 0); // timestamp is not serialized
    assert_eq!(MyDataV1::DATA_FORMAT, DataFormat::Struct4);
}

#[test]
fn check_simple_serde_unique_id() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        u: UniqueID,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            u: UniqueID::with_value(0x8899AABBCCDDEEFF),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t.x, t2.x);
    assert_eq!(t.a, t2.a);
    assert_eq!(t.d.a, t2.d.a);
    assert_eq!(t.d.b, t2.d.b);
    assert_eq!(t2.d.u.value(), 0); // unique_id is not serialized
    assert_eq!(MyDataV1::DATA_FORMAT, DataFormat::Struct4);
}

#[test]
fn check_nested_2_levels() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelTwo {
        a: bool,
        l: Vec<i8>,
    }
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelOne {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        #[flat_message_item(align = 4, kind = struct)]
        e: LevelTwo,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: LevelOne,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: LevelOne {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
            e: LevelTwo {
                a: true,
                l: vec![1, 2, 3],
            },
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
    assert_eq!(LevelOne::DATA_FORMAT, DataFormat::Struct4);
    assert_eq!(LevelTwo::DATA_FORMAT, DataFormat::Struct4);
}

#[test]
fn check_nested_2_levels_second_level_128_alignament() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelTwo {
        a: bool,
        l: Vec<i128>,
    }
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelOne {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        #[flat_message_item(align = 16, kind = struct)]
        e: LevelTwo,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 16, kind = struct)]
        d: LevelOne,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: LevelOne {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
            e: LevelTwo {
                a: true,
                l: vec![1, 2, 3],
            },
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
    // both should be 16 bytes alignament
    assert_eq!(LevelOne::DATA_FORMAT, DataFormat::Struct16);
    assert_eq!(LevelTwo::DATA_FORMAT, DataFormat::Struct16);
}
