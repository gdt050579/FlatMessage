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
    assert_eq!(s.b2.capacity(), 4);

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
    assert_eq!(s.b2.capacity(), 2);
}

#[test]
fn check_buffer_i8_serde_option() {
    // TODO
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
