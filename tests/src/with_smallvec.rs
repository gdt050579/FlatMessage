use crate::*;
use flat_message::*;

use smallvec::SmallVec;

#[test]
fn check_buffer_i8_serde_smallvec() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [i8],
        b2: smallvec::SmallVec<[i8; 2]>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[-10i8, -20, -30],
        b2: SmallVec::from_slice(&[1, 2, 3, 4]),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(ds.b2.capacity(), 4);
    assert_eq!(ds.b2.spilled(), true);

    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[-10i8, -20, -30],
        b2: SmallVec::from_slice(&[1]),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(ds.b2.capacity(), 2);
    assert_eq!(ds.b2.spilled(), false);
}

#[test]
fn check_option_smallvec() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<smallvec::SmallVec<[u32; 2]>>,
        v2: Option<smallvec::SmallVec<[bool; 2]>>,
    }
    let mut v = Storage::default();
    let s = Test {
        v1: Some(SmallVec::from_slice(&[1, 2])),
        v2: None,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = Test::deserialize_from(&v).unwrap();
    assert_eq!(s.v1, ds.v1);
    assert_eq!(s.v2, ds.v2);
    assert_eq!(ds.v1.map(|v| v.spilled()), Some(false));
    assert_eq!(ds.v2, None);
}

#[test]
fn check_option_smallvec_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<smallvec::SmallVec<[u8; 2]>>,
        v2: Option<smallvec::SmallVec<[String; 2]>>,
        v3: Option<bool>,
    }
    let t = Test {
        v1: Some(SmallVec::from_slice(&[1, 2, 3, 4])),
        v2: Some(Smallvec::from(&["Hello".to_string(), "xyz".to_string()])),
        v3: None,
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header
            4, // elements in v1 (4)
            1, 2, 3, 4, // v1 elements
            2, // elements in v2 (2)
            5, 72, 101, 108, 108, 111, // v2[0] (size + Hello)
            3, 120, 121, 122, // v2[1] (size + xyz)
            129, 70, 74, 148, // hash for v1
            13, 73, 74, 150, // hash for v3
            142, 75, 74, 151, // hash for v2
            8,   // offset of v1
            0,   // offset of v3 (NOne = 0)
            13   // offset of v2
        ]
    );
}

#[test]
fn check_enum_smallvec_u8bits() {
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
        color: SmallVec<[Color; 3]>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: SmallVec::from_slice(&[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ]),
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
fn check_smallvec_object() {
    use smallvec::smallvec;
    use smallvec::SmallVec;

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: SmallVec<[String; 2]>,
        s2: SmallVec<[&'a str; 2]>,
    }

    {
        let mut v = Storage::default();
        let t = Test {
            s1: smallvec!["Hello".to_string(), "World".to_string()],
            s2: smallvec!["abc", "xyz"],
        };
        {
            t.serialize_to(&mut v, Config::default()).unwrap();
            let ds = Test::deserialize_from(&v).unwrap();
            assert_eq!(t, ds);
            assert_eq!(t.s1.spilled(), false);
            assert_eq!(t.s2.spilled(), false);
        }
    }

    {
        let mut v = Storage::default();
        let t = Test {
            s1: smallvec!["Hello".to_string(), "World".to_string(), "Everyone".to_string()],
            s2: smallvec!["abc", "xyz", "123"],
        };
        {
            t.serialize_to(&mut v, Config::default()).unwrap();
            let ds = Test::deserialize_from(&v).unwrap();
            assert_eq!(t, ds);
            assert_eq!(t.s1.spilled(), true);
            assert_eq!(t.s2.spilled(), true);
        }
    }
}
