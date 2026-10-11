use crate::*;
use flat_message::*;

#[test]
fn check_packed_no_alignament_required() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        x: i32,
        y: u32,
        label: String,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct Test {
        #[flat_message_item(kind = packed, align = 1)]
        point: Point,
        x: String,
    }
    validate_correct_serde(Test {
        point: Point {
            x: 10,
            y: 20,
            label: "Test".to_string(),
        },
        x: "Hello".to_string(),
    });
}

#[test]
fn check_packed_no_alignament_required_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        x: i32,
        y: u32,
        label: String,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        #[flat_message_item(kind = packed, align = 1)]
        point: Point,
        x: String,
    }
    let t = Test {
        point: Point {
            x: 10,
            y: 20,
            label: "Test".to_string(),
        },
        x: "Hello".to_string(),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            179, 7, 224, 175, // Point structure hash
            10, 0, 0, 0, // Point::x = 10
            20, 0, 0, 0, // Point::y = 20
            4, 84, 101, 115, 116, // Point::label = "Test" (size 4)
            5, 72, 101, 108, 108, 111, // label = "Hello"
            0,   // padding
            40, 108, 174, 24, // hash for point parameter
            14, 80, 12, 253, // hash for x parameter
            8,   // offset of point
            25   // offset of x parameter
        ]
    );
}

#[test]
fn check_packed_4_bytes_alignament_required() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Data {
        data: Vec<u32>,
        idx: u8,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        x: i8,
        y: i8,
        #[flat_message_item(kind = packed, align = 4)]
        d1: Data,
        #[flat_message_item(kind = packed, align = 4)]
        d2: Data,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = packed, align = 4)]
        point: Point,
        x: String,
    }
    validate_correct_serde(Test {
        point: Point {
            x: 10,
            y: 20,
            d1: Data {
                data: vec![1, 2, 3],
                idx: 1,
            },
            d2: Data {
                data: vec![4, 5, 6],
                idx: 2,
            },
        },
        x: "Hello".to_string(),
    });
}

#[test]
fn check_packed_4_bytes_alignament_required_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Data {
        data: Vec<u32>,
        idx: u8,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        x: i8,
        y: i8,
        #[flat_message_item(kind = packed, align = 4)]
        d1: Data,
        #[flat_message_item(kind = packed, align = 4)]
        d2: Data,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = packed, align = 4)]
        point: Point,
        x: String,
    }
    let t = Test {
        point: Point {
            x: 10,
            y: 20,
            d1: Data {
                data: vec![1, 2, 3],
                idx: 1,
            },
            d2: Data {
                data: vec![4, 5, 6],
                idx: 2,
            },
        },
        x: "Hello".to_string(),
    };
    //println!("point size:{}", flat_message::SerDe::size(&t.point));
    //println!("d1:{}", flat_message::SerDe::size(&t.point.d1));
    //println!("d2:{}", flat_message::SerDe::size(&t.point.d2));
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, 203, 182, 210, 213, // Point structure hash
            151, 27, 83, 238, // Hash for Data (Point::d1 field)
            3, 0, 0, 0, // 3 elements in d1::data
            1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0,
            0, // d1::data[0] = 1, d1::data[1] = 2, d1::data[2] = 3
            1, // d1::idx = 1
            0, 0, 0, // padding
            151, 27, 83, 238, // Hash for Data (Point::d2 field)
            3, 0, 0, 0, // 3 elements in d2::data
            4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0,
            0,  // d2::data[0] = 4, d2::data[1] = 5, d2::data[2] = 6
            2,  // d2::idx = 2
            10, // Point::x = 10
            20, // Point::y = 20
            5, 72, 101, 108, 108, 111, // Test::x = "Hello"
            0, 0, 0, // padding
            42, 108, 174, 24, // hash for point parameter
            14, 80, 12, 253, // hash for x parameter
            8,   // offset of point
            59,  // offset of x parameter
        ]
    );
}
