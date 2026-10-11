use super::*;
use flat_message::*;

mod v1 {
    use flat_message::*;
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    pub enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    pub struct Test {
        pub x: u8,
        pub y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        pub v: MyEnum,
    }
    impl Test {
        pub fn new() -> Self {
            Test {
                x: 1,
                y: 2,
                v: MyEnum::String(String::from("Hello")),
            }
        }
    }
}

mod v1_sealed {
    use flat_message::*;
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    #[sealed]
    pub enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    pub struct Test {
        pub x: u8,
        pub y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        pub v: MyEnum,
    }
    impl Test {
        pub fn new() -> Self {
            Test {
                x: 1,
                y: 2,
                v: MyEnum::String(String::from("Hello")),
            }
        }
    }
}

mod v2 {
    use flat_message::*;
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    pub enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    pub struct Test {
        pub x: u8,
        pub y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        pub v: MyEnum,
    }
    impl Test {
        pub fn new() -> Self {
            Test {
                x: 1,
                y: 2,
                v: MyEnum::String(String::from("Hello")),
            }
        }
    }
}

#[test]
fn check_align_16() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u128>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 16)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::DWord(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Vector(vec![1, 2, 3]),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::SimpleVariant,
    });
}

#[test]
fn check_align_16_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u128>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 16)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    };

    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            0, 0, 0, 0, 0, 0, 0, 0, // alignament to 16 bytes
            220, 228, 14, 121, // hash over the variant MyEnum
            1, 53, 242, 61, // hash over the variant Byte (1 = u8)
            3,  // value of the variant Byte (3)
            2, 0, // y = 2
            1, // x = 1
            39, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            16,  // offset fro Test::v (offset 16)
            25,  // offset for Test::y (offset 25)
            27,  // offset for Test::x (offset 27)
        ]
    );
}

#[test]
fn check_align_1() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::DWord(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Vector(vec![1, 2, 3]),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::SimpleVariant,
    });
}

#[test]
fn check_align_2() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u16>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 2)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::DWord(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Vector(vec![1, 2, 3]),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::SimpleVariant,
    });
}

#[test]
fn check_align_4() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u32>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::DWord(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Vector(vec![1, 2, 3]),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::SimpleVariant,
    });
}

#[test]
fn check_align_8() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u64>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 8)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::DWord(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::Vector(vec![1, 2, 3]),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::SimpleVariant,
    });
}

#[test]
fn check_align_4_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u32>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    };

    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            220, 228, 14, 121, // hash over the variant MyEnum
            14, 183, 18, 153, // hash over the variant String (14 = String)
            5, 72, 101, 108, 108, 111, // value of the variant String ("Hello") size = 5
            2, 0, // Test::y = 2
            1, // Test::x = 1
            0, 0, 0, // padding to 4 bytes
            37, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // offset for Test::v (offset 8)
            22,  // offset for Test::y (offset 22)
            24,  // offset for Test::x (offset 24)
        ]
    );
}

#[test]
fn check_combo_enum_and_flags() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u8);
    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = flags, repr = u8)]
        FlagsEntry(Flags),
        #[flat_message_item(kind = enum, repr = u32)]
        MetricEntry(Metric),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyVariant,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::StringValue("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::FlagsEntry(Flags::A),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::FlagsEntry(Flags::B | Flags::C),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::FlagsEntry(Flags::A | Flags::B | Flags::C),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::MetricEntry(Metric::Km),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::MetricEntry(Metric::Liters),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::MetricEntry(Metric::Temperature),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::MetricEntry(Metric::Pressue),
    });
}

#[test]
fn check_combo_enum_and_flags_repr_flags() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u8);
    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = flags, repr = u8)]
        FlagsEntry(Flags),
        #[flat_message_item(kind = enum, repr = u32)]
        MetricEntry(Metric),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyVariant,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyVariant::FlagsEntry(Flags::A | Flags::B | Flags::C),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            10, 35, 89, 217, // hash over the variant MyVariant
            27, 133, 93, 57, // hash over the variant FlagsEntry (27 = Flags8 (8 bits))
            190, 110, 196, 202, // hash over the type Flags
            7,   // value of variant FlagsEntry (7 = A | B | C)
            2, 0, // Test::y = 2
            1, // Test::x = 1
            35, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // offset for Test::v (offset 8)
            21,  // offset for Test::y (offset 21)
            23,  // offset for Test::x (offset 23)
        ]
    );
}

#[test]
fn check_combo_enum_and_flags_repr_enum() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u8);
    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = flags, repr = u8)]
        FlagsEntry(Flags),
        #[flat_message_item(kind = enum, repr = u32)]
        MetricEntry(Metric),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyVariant,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyVariant::MetricEntry(Metric::Km),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            10, 35, 89, 217, // hash over the variant MyVariant
            21, 31, 35, 190, // hash over the variant MetricEntry (21 = EnumU32)
            213, 43, 122, 128, // hash over the type Metric
            1, 0, 0, 0, // value of variant MetricEntry (1 = Km)
            2, 0, // Test::y = 2
            1, // Test::x = 1
            0, // padding
            35, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // offset for Test::v (offset 8)
            24,  // offset for Test::y (offset 24)
            26,  // offset for Test::x (offset 26)
        ]
    );
}

#[test]
fn check_combo_structs() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyStruct1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyStruct2 {
        a: u8,
        b: Vec<u32>,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = struct, align = 4)]
        S1(MyStruct1),
        #[flat_message_item(kind = struct, align = 4)]
        S2(MyStruct2),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyVariant,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::Byte(3),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::StringValue("Hello".to_string()),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::S1(MyStruct1 {
            a: 1,
            b: 2,
            c: 3,
            d: "Hello".to_string(),
        }),
    });
    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyVariant::S2(MyStruct2 {
            a: 1,
            b: vec![2, 3, 4],
        }),
    });
}

#[test]
fn check_combo_structs_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyStruct1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyStruct2 {
        a: u8,
        b: Vec<u32>,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = struct, align = 4)]
        S1(MyStruct1),
        #[flat_message_item(kind = struct, align = 4)]
        S2(MyStruct2),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyVariant,
    }
    let t = Test {
        x: 1,
        y: 2,
        v: MyVariant::S1(MyStruct1 {
            a: 1,
            b: 2,
            c: 3,
            d: "Hello".to_string(),
        }),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            10, 35, 89, 217, // hash over the variant MyVariant
            32, 28, 145, 129, // hash over the variant S1 (32 = Struct4)
            177, 148, 3, 133, // hash over the type MyStruct1
            16, 44, 0, 0, // number of fields = 4 (16>>2), size = 44 bytes
            5, 72, 101, 108, 108, 111, // value of the variant Test::v::S1::d ("Hello")
            1,   // Test::v::S1::a = 1 (u8)
            3, 0, // Test::v::S1::c = 3 (u16)
            2, 0, 0, 0, // Test::v::S1::b = 2 (u32)
            0, 0, 0, // padding to 4 bytes
            14, 36, 12, 225, // hash for Test::v::S1::d
            1, 41, 12, 228, // hash for Test::v::S1::a
            2, 44, 12, 230, // hash for Test::v::S1::c
            3, 45, 12, 231, // hash for Test::v::S1::b
            8,   // offset for Test::v::S1::d (offset 8)
            14,  // offset for Test::v::S1::a (offset 14)
            15,  // offset for Test::v::S1::c (offset 15)
            17,  // offset for Test::v::S1::b (offset 17)
            2, 0, // valu of Test::y = 2
            1, // value of Test::x = 1
            0, // padding to 4 bytes
            37, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // offset for Test::v (offset 8)
            60,  // offset for Test::y (offset 60)
            62,  // offset for Test::x (offset 62)
        ]
    );
}

#[test]
fn check_option_with_none() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(Option<String>),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String(None),
    });
}

#[test]
fn check_option_with_none_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(Option<String>),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::String(None),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            220, 228, 14, 121, // hash for MyEnum
            78, 183, 18,
            153, // hash for MyEnum::String (String variant), 78 = 0x4E -> 0x40 (Option::None) | 0x0E (String)
            2, 0, // Test::y
            1, // Test::x
            0, // padding
            35, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // Offset of Test::v
            16,  // Offset of Test::y
            18   // Offset of Test::x
        ]
    );
}

#[test]
fn check_option_with_some() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(Option<String>),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    validate_correct_serde(Test {
        x: 1,
        y: 2,
        v: MyEnum::String(Some(String::from("Hello"))),
    });
}

#[test]
fn check_option_with_some_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(Option<String>),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::String(Some(String::from("Hello"))),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header
            220, 228, 14, 121, // hash for MyEnum
            14, 183, 18, 153, // hash for MyEnum::String (14 = String) or Some(String)
            5, 72, 101, 108, 108, 111, // Test::v::String -> Hello (size = 5)
            2, 0, // Test::y
            1, // Test::x
            0, 0, 0, // padding
            35, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8,   // Offset of Test::v
            22,  // Offset of Test::y
            24   // Offset of Test::x
        ]
    );
}

#[test]
fn chcck_v1_in_v2() {
    let t_v1 = v1::Test::new();
    let mut s = Storage::default();
    t_v1.serialize_to(&mut s, Config::default()).unwrap();
    let t_v2 = v2::Test::deserialize_from(&s).unwrap();
    assert_eq!(t_v1.x, t_v2.x);
    assert_eq!(t_v1.y, t_v2.y);
    match t_v1.v {
        v1::MyEnum::String(v) => assert_eq!(&v, "Hello"),
        _ => panic!("Expected String variant"),
    }
    match t_v2.v {
        v2::MyEnum::String(v) => assert_eq!(&v, "Hello"),
        _ => panic!("Expected String variant"),
    }
}

#[test]
fn chcck_v2_in_v1_same_fields() {
    let t_v2 = v2::Test::new();
    let mut s = Storage::default();
    t_v2.serialize_to(&mut s, Config::default()).unwrap();
    let t_v1 = v1::Test::deserialize_from(&s).unwrap();
    assert_eq!(t_v2.x, t_v1.x);
    assert_eq!(t_v2.y, t_v1.y);
    match t_v2.v {
        v2::MyEnum::String(v) => assert_eq!(&v, "Hello"),
        _ => panic!("Expected String variant"),
    }
    match t_v1.v {
        v1::MyEnum::String(v) => assert_eq!(&v, "Hello"),
        _ => panic!("Expected String variant"),
    }
}

#[test]
fn chcck_v1_sealed_in_v1() {
    let t_v1_sealed = v1_sealed::Test::new();
    let mut s = Storage::default();
    t_v1_sealed.serialize_to(&mut s, Config::default()).unwrap();
    let t_v1 = v1::Test::deserialize_from(&s);
    match t_v1 {
        Ok(_) => {
            panic!("Expected error - sealed enum can't be deserialized to v1");
        }
        Err(e) => {
            match e {
                Error::FailToDeserialize(_) => {
                    // Ok
                }
                _ => {
                    panic!("Expected error - sealed enum can't be deserialized to v1");
                }
            }
        }
    }
}

#[test]
fn chcck_v1_in_v1_sealed() {
    let t_v1 = v1::Test::new();
    let mut s = Storage::default();
    t_v1.serialize_to(&mut s, Config::default()).unwrap();
    let t_v1_sealed = v1_sealed::Test::deserialize_from(&s);
    match t_v1_sealed {
        Ok(_) => {
            panic!("Expected error - v1 enum can't be deserialized to v1_sealed");
        }
        Err(e) => {
            match e {
                Error::FailToDeserialize(_) => {
                    // Ok
                }
                _ => {
                    panic!("Expected error - sealed enum can't be deserialized to v1");
                }
            }
        }
    }
}

#[test]
fn chcck_v2_in_v1_new_added_fields() {
    let t_v2 = v2::Test {
        x: 1,
        y: 2,
        // MyEnum::Vector is not present in v1
        v: v2::MyEnum::Vector(vec![1, 2, 3]),
    };
    let mut s = Storage::default();
    t_v2.serialize_to(&mut s, Config::default()).unwrap();
    let t_v1 = v1::Test::deserialize_from(&s);
    match t_v1 {
        Ok(_) => {
            panic!("Expected error - v2 enum can't be deserialized to v1");
        }
        Err(e) => {
            match e {
                Error::FailToDeserialize(_) => {
                    // Ok
                }
                _ => {
                    panic!("Expected error - v2 enum can't be deserialized to v1");
                }
            }
        }
    }
}

#[test]
fn check_lifetime() {
    #[derive(FlatMessageStruct)]
    pub struct VariantA {
        pub a: u32,
        pub b: u8,
    }

    #[derive(FlatMessageStruct)]
    pub struct VariantB<'lifetimefortest> {
        pub a: &'lifetimefortest str,
        pub b: &'lifetimefortest str,
    }

    #[derive(FlatMessageVariant)]
    pub enum Variant<'lifetimefortest> {
        #[flat_message_item(kind = struct, align = 4)]
        A(VariantA),
        #[flat_message_item(kind = struct, align = 4)]
        B(VariantB<'lifetimefortest>),
        C,
        D,
    }

    #[derive(FlatMessage)]
    #[flat_message_options(optimized_unchecked_code = false)]
    pub struct Thing<'lifetimefortest> {
        pub d: &'lifetimefortest str,
        #[flat_message_item(kind = variant, align = 4)]
        pub variant: Variant<'lifetimefortest>,
    }

    let t = Thing {
        d: "Hello",
        variant: Variant::A(VariantA { a: 1, b: 2 }),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t_deserialized = Thing::deserialize_from(&s).unwrap();
    assert_eq!(t_deserialized.d, "Hello");
    match t_deserialized.variant {
        Variant::A(v) => {
            assert_eq!(v.a, 1);
        }
        _ => {
            panic!("Expected A variant");
        }
    }
}
