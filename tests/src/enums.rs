use flat_message::*;

mod v1 {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        pub color: Color,
    }
}
mod v2 {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
        Yellor = 200,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        pub color: Color,
    }
}
mod v3 {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    #[sealed]
    pub enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        pub color: Color,
    }
}

#[test]
fn check_enum() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        color: Color,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, 237, 103, 151, 167, 10, 123, 0, 0, 19, 98, 126, 61, 1, 211,
            94, 66, 8, 13
        ]
    );
}

#[test]
fn check_enum_add_variant() {
    let mut v = Storage::default();
    let s = v1::TestStruct {
        value: 123,
        color: v1::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = v2::TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color as u8, ds.color as u8);
}

#[test]
fn check_enum_add_variant_sealed() {
    let mut v = Storage::default();
    let s = v3::TestStruct {
        value: 123,
        color: v3::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = v2::TestStruct::deserialize_from(&v);
    match ds {
        Err(flat_message::Error::FailToDeserialize(_)) => {}
        _ => panic!("Invalid error - expected InvalidEnumVariant"),
    }
}

#[test]
fn check_enum_add_variant_sealed_unchecked() {
    let mut v = Storage::default();
    let s = v3::TestStruct {
        value: 123,
        color: v3::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    // seald argument is not checked
    let ds = unsafe { v2::TestStruct::deserialize_from_unchecked(&v).unwrap() };
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color as u8, ds.color as u8);
}

#[test]
fn check_enum_slice_u8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            70, 76, 77, 1, 2, 0, 0, 0, // TestStruct::color
            // Hash for Color
            237, 103, 151, 167, // number of elements in TestStruct::color
            5,   // u8 value for TestStruct::color
            10, 100, 1, 10, 100, // value of TestStruct::value
            123, // alignament padding (to 4 bytes)
            0,   // Hash for color
            147, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 18
        ]
    );
}

#[test]
fn check_enum_slice_i8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i8)]
    enum Color {
        Red = 1,
        Green = -10,
        Blue = -100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = i8, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u16bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u16)]
    enum Color {
        Red = 1234,
        Green = 12345,
        Blue = 2,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u16, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            70, 76, 77, 1, 2, 0, 0, 0, // Hash for Color
            237, 103, 151, 167, // number of items in TestStruct::color (u16)
            5, 0, // 5 items of TestStruct::color (u16) each
            57, 48, 2, 0, 210, 4, 57, 48, 2, 0,   // TestStruct::value
            123, // alignament padding (to 4 bytes)
            0, 0, 0, // Hash for color
            148, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 24
        ]
    );
}

#[test]
fn check_enum_slice_i16bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i16)]
    enum Color {
        Red = 1234,
        Green = -12345,
        Blue = 2,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = i16, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u32bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Color {
        Red = 0xFF00FF00,
        Green = 0x00FF00FF,
        Blue = 0xFEFEFEFE,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u32, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_i32bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i32)]
    enum Color {
        Red = -12345678,
        Green = 1,
        Blue = 12345678,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = i32, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u64bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u64)]
    enum Color {
        Red = 0xFF00FF0012345678,
        Green = 0x00FF00FF11223344,
        Blue = 0xFEFEFEFE99887766,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u64, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_i64bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i64)]
    enum Color {
        Red = 0xFF00FF00123876,
        Green = -11111111111111111,
        Blue = -87614876518,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = i64, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_vec_u8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        color: Vec<Color>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: [
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ]
        .to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            70, 76, 77, 1, 2, 0, 0, 0, // TestStruct::color
            // Hash for Color
            237, 103, 151, 167, // number of elements in TestStruct::color
            5,   // u8 value for TestStruct::color
            10, 100, 1, 10, 100, // value of TestStruct::value
            123, // alignament padding (to 4 bytes)
            0,   // Hash for color
            147, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 18
        ]
    );
}

#[test]
fn check_enum_vec_and_slice() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        color: Vec<Color>,
        #[flat_message_item(repr = u8, kind = enum)]
        s_color: &'a [Color],
        #[flat_message_item(repr = u32, kind = enum)]
        metric: Vec<Metric>,
    }
    let mut v = Storage::default();
    let large_vec = [Color::Red, Color::Green, Color::Blue]
        .repeat(1000)
        .to_vec();
    let s = TestStruct {
        value: 123,
        color: [
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ]
        .to_vec(),
        s_color: large_vec.as_slice(),
        metric: [Metric::Km, Metric::Liters, Metric::Temperature]
            .repeat(2000)
            .to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(s.s_color, ds.s_color);
    assert_eq!(s.metric, ds.metric);
}

#[test]
fn check_enum_vec_and_slice_u32align() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        s_color: &'a [Color],
        #[flat_message_item(repr = u32, kind = enum)]
        metric: Vec<Metric>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        s_color: &[Color::Red, Color::Green, Color::Blue],
        metric: [Metric::Km, Metric::Liters, Metric::Temperature].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.s_color, ds.s_color);
    assert_eq!(s.metric, ds.metric);
    assert_eq!(
        v.as_slice(),
        [
            70, 76, 77, 1, 3, 0, 0, 0, 213, 43, 122, 128, 3, 0, 0, 0, 1, 0, 0, 0, 128, 150, 152, 0,
            0, 202, 154, 59, 123, 237, 103, 151, 167, 3, 1, 10, 100, 0, 0, 0, 1, 211, 94, 66, 149,
            67, 175, 201, 147, 206, 4, 209, 28, 8, 29
        ]
    );
}

#[test]
fn check_enum_from_module() {
    mod m1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        value: u8,
        #[flat_message_item(repr = u8, kind = enum)]
        color: m1::Color,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: m1::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, 237, 103, 151, 167, 10, 123, 0, 0, 19, 98, 126, 61, 1, 211,
            94, 66, 8, 13
        ]
    );
}
