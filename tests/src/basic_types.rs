use crate::*;
use flat_message::*;

#[test]
fn check_int() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    });
}

#[test]
fn check_int_negative() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: -1,
        v2: -2,
        v3: -3,
        v4: -4,
    });
}

#[test]
fn check_int_min() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: i8::MIN,
        v2: i16::MIN,
        v3: i32::MIN,
        v4: i64::MIN,
    });
}

#[test]
fn check_int_max() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: i8::MAX,
        v2: i16::MAX,
        v3: i32::MAX,
        v4: i64::MAX,
    });
}

#[test]
fn check_uint() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    });
}

#[test]
fn check_uint_min() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: u8::MIN,
        v2: u16::MIN,
        v3: u32::MIN,
        v4: u64::MIN,
    });
}

#[test]
fn check_uint_max() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: u8::MAX,
        v2: u16::MAX,
        v3: u32::MAX,
        v4: u64::MAX,
    });
}

#[test]
fn check_int_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    let mut v = Storage::default();
    let s = Test {
        v1: 0x55,
        v2: 0x6677,
        v3: 0x11223344,
        v4: 0x55667788_99AABBCC,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 4, 0, 0, 0,  // Heder
            85, // v1 - 0x55
            68, 51, 34, 17, // v3 = 0x11223344
            119, 102, // v2 = 0x6677
            204, 187, 170, 153, 136, 119, 102, 85, // v4 = 0x55667788_99AABBCC
            0,  // padding
            6, 70, 74, 148, // hash for v1
            8, 73, 74, 150, // hash for v3
            7, 75, 74, 151, // hash for v2
            9, 78, 74, 153, // hash for v4
            8,   // offset for v1
            9,   // offset for v3
            13,  // offset of v2
            15,  // offset of v4
        ]
    );
}

#[test]
fn check_uint_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    let mut v = Storage::default();
    let s = Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 4, 0, 0, 0, // Header
            1, // v1 - 1
            3, 0, 0, 0, // v3 = 3
            2, 0, // v2 = 2
            4, 0, 0, 0, 0, 0, 0, 0, // v4 = 4
            0, // padding
            6, 70, 74, 148, // hash for v1
            8, 73, 74, 150, // hash for v3
            7, 75, 74, 151, // hash for v2
            9, 78, 74, 153, // hash for v4
            8,   // offset for v1 - 8
            9,   // offset for v3 - 9
            13,  // offset for v2 - 13
            15   // offset for v4 - 15
        ]
    );
}

#[test]
fn check_derive() {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, FlatMessage)]
    struct TestStruct {
        a: i32,
        b: bool,
        c: u16,
    }
    validate_correct_serde(TestStruct {
        a: 1,
        b: true,
        c: 123,
    });
}

#[test]
fn check_bool() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: bool,
        v2: bool,
        v3: bool,
    }
    validate_correct_serde(Test {
        v1: true,
        v2: false,
        v3: true,
    });
}

#[test]
fn check_float() {
    #[derive(Debug, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: f32,
        v2: f64,
    }
    let mut v = Storage::default();
    let t = Test {
        v1: 0.625f32,
        v2: 1234.75f64,
    };
    t.serialize_to(&mut v, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&v).unwrap();
    assert_eq!(t.v1, deserialized.v1);
    assert_eq!(t.v2, deserialized.v2);
}

#[test]
fn check_buffer_i8_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [i8],
        b2: Vec<i8>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[-10i8, -20, -30],
        b2: [1, 2, 3, 4].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
}

#[test]
fn check_buffer_i8_repr() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [i8],
        b2: Vec<i8>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[-10i8, -20, -30],
        b2: [1, 2, 3, 4].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 8, // Header
            64, 226, 1, 0, // value = 123456
            3, 246, 236, 226, // b1 = [-10, -20, -30] (3 elements)
            4, 1, 2, 3, 4, // b2 = [1, 2, 3, 4] (4 elements)
            0, 0, 0, // padding
            3, 211, 94, 66, // hash for value
            134, 36, 44, 140, // hash for b1
            134, 41, 44, 143, // hash for b2
            8,   // offset for value - 8
            12,  // offset for b1 - 12
            16,  // offset for b2 - 16
            64, 213, 197, 45 // name hash for TestStruct
        ]
    );
}

#[test]
fn check_serde_buffer_u8() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [u8],
        b2: Vec<u8>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[200, 201, 202, 203, 255, 255, 255],
        b2: [1, 2, 3, 4, 6, 7, 8, 9, 10].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
}

#[test]
fn check_buffer_u16_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        b2: Vec<u16>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        b2: [1, 2, 3].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            3, // size of b2 (3 elements)
            0, 1, 0, 2, 0, 3, // b2 = [1, 2, 3] (3 elements)
            0, // padding
            130, 41, 44, 143, // hash for b2
            8    // offset for b2 - 8
        ]
    );
}

#[test]
fn check_buffer_u16_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [u16],
        b2: Vec<u16>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[200, 201, 202, 203, 255, 255, 255],
        b2: [1, 2, 3, 4, 6, 7, 8, 9, 10].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
}

#[test]
fn check_buffer_i16_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [i16],
        b2: Vec<i16>,
        name: String,
        surname: &'a str,
        checked: bool,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[200, 201, 202, 203, 255, 255, 255],
        b2: [1, 2, 3, 4, 6, 7, 8, 9, 10].to_vec(),
        name: "John".to_string(),
        surname: "Doe",
        checked: true,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
    assert_eq!(s.checked, ds.checked);
}

#[test]
fn check_buffer_i32_u32_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [i32],
        b2: Vec<i32>,
        b3: &'a [u32],
        b4: Vec<u32>,
        name: String,
        surname: &'a str,
        checked: bool,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[200, 201, 202, 203, 255, 255, 255],
        b2: [-1, 2, -3, 4, -6, 7, -8, 9, -10].to_vec(),
        b3: &[10, 20, 30, 40],
        b4: [1, 2, 3, 4, 6, 7, 8, 9, 10].to_vec(),
        name: "John".to_string(),
        surname: "Doe",
        checked: true,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(s.b3, ds.b3);
    assert_eq!(s.b4, ds.b4);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
    assert_eq!(s.checked, ds.checked);
}

#[test]
fn check_buffer_f32_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [f32],
        b2: Vec<f32>,
        name: String,
        surname: &'a str,
        checked: bool,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[1.2f32, 2.3, 3.4, 4.5, 6.7, 7.8, 8.9],
        b2: [-12345.1234f32, 123.123, 1000.0, 0.0].to_vec(),
        name: "John".to_string(),
        surname: "Doe",
        checked: true,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
    assert_eq!(s.checked, ds.checked);
}

#[test]
fn check_serde_64_bits_buffers() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [f64],
        b2: Vec<f64>,
        b3: &'a [i64],
        b4: Vec<i64>,
        b5: &'a [u64],
        b6: Vec<u64>,
        name: String,
        surname: &'a str,
        checked: bool,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[1.2f64, 2.3, 3.4, 4.5, 6.7, 7.8, 8.9],
        b2: [-12345.1234f64, 123.123, 1000.0, 0.0].to_vec(),
        b3: &[-1, 2, -3, 0x1234567890, -6, 7, -8, i64::MIN, -10, i64::MAX],
        b4: [1, -2, 300, 0x1234567890, -678910876, i64::MIN, i64::MAX].to_vec(),
        b5: &[0, 100, 100_000, 100_000_000, 100_000_000_000, u64::MAX],
        b6: [u64::MAX, 0, 0xFFFF_FFFF_FFFF, 0xEEEE_EEEE_EEEE_EEEE].to_vec(),
        name: "John".to_string(),
        surname: "Doe",
        checked: true,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
    assert_eq!(s.b3, ds.b3);
    assert_eq!(s.b4, ds.b4);
    assert_eq!(s.b5, ds.b5);
    assert_eq!(s.b6, ds.b6);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
    assert_eq!(s.checked, ds.checked);
}

#[test]
fn check_bool_serde() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        value: u32,
        b1: &'a [bool],
        b2: Vec<bool>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b1: &[true, false, true, true, false, false, true],
        b2: [true, false, false, true, false, true, true, true, false].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b1, ds.b1);
    assert_eq!(s.b2, ds.b2);
}

#[test]
fn check_simple_struct() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Point {
        x: i32,
        y: i32,
    }
    validate_correct_serde(Point { x: 10, y: 20 });
}

#[test]
fn check_aliganemnt_order_u32_u16_string() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        buf_u32_aligned: &'a [u32],
        list_u16_aligned: Vec<u16>,
        string_u8_aligned: String,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        buf_u32_aligned: &[1, 2, 3, 4],
        list_u16_aligned: [1, 2, 3].to_vec(),
        string_u8_aligned: "Hello".to_string(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    // order in the buffer should be: buf_u32_aligned, list_u16_aligned, string_u8_aligned
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header
            4, 0, 0, 0, // size of buf_u32_aligned (4 elements) - aligned to 4 bytes
            1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, // buf_u32_aligned = [1, 2, 3, 4]
            3, 0, // size of list_u16_aligned (3 elements) - aligned to 2 bytes
            1, 0, 2, 0, 3, 0, // list_u16_aligned = [1, 2, 3]
            5, // size of string_u8_aligned (5 bytes) - aligned to 1 byte
            72, 101, 108, 108, 111, // string_u8_aligned = "Hello"
            0, 0, // padding
            14, 159, 54, 27, // hash for string_u8_aligned
            131, 216, 51, 208, // hash for buf_u32_aligned
            130, 226, 119, 250, // hash for list_u16_aligned
            36,  // offset for string_u8_aligned - 36
            8,   // offset for buf_u32_aligned - 8
            28   // offset for list_u16_aligned - 28
        ]
    );
}

#[test]
fn check_serde_128_bits_buffers() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        b3: &'a [i128],
        b4: Vec<i128>,
        b5: &'a [u128],
        b6: Vec<u128>,
        name: String,
        surname: &'a str,
        checked: bool,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        b3: &[
            -1,
            2,
            -3,
            0x1234567890,
            -6,
            7,
            -8,
            i128::MIN,
            -10,
            i128::MAX,
        ],
        b4: [1, -2, 300, 0x1234567890, -678910876, i128::MIN, i128::MAX].to_vec(),
        b5: &[0, 100, 100_000, 100_000_000, 100_000_000_000, u128::MAX],
        b6: [u128::MAX, 0, 0xFFFF_FFFF_FFFF, 0xEEEE_EEEE_EEEE_EEEE].to_vec(),
        name: "John".to_string(),
        surname: "Doe",
        checked: true,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.b3, ds.b3);
    assert_eq!(s.b4, ds.b4);
    assert_eq!(s.b5, ds.b5);
    assert_eq!(s.b6, ds.b6);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
    assert_eq!(s.checked, ds.checked);
}

#[test]
fn check_u128_repr() {
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        d: Vec<u128>,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: vec![1, 2, 3],
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, // alignament for d (16 bytes)            
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // number of elements in d
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // d[0]
            2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // d[1]
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // d[2]
            5, // Test::a
            1, 0, // Test::x
            0, // Padding
            133, 36, 12, 225, // Hash for Test::d
            1, 41, 12, 228, // Hash for Test::a
            1, 80, 12, 253, // Hash for Test::x
            16, // Offset of Test::d
            80, // Offset of Test::a
            81  // Offset of Test::x
        ]
    );
}

#[test]
fn check_serde_128_bits_alignament() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        b6: Vec<u128>,
        b4: Vec<u64>,
        b5: Vec<u32>,
        name: String,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        b6: [1, 2, 3].to_vec(),
        b4: [10, 20].to_vec(),
        b5: [40, 41, 42, 43].to_vec(),
        name: "Hello".to_string(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let expected = &[
        70, 76, 77, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 10,
        0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0,
        0, 0, 43, 0, 0, 0, 5, 72, 101, 108, 108, 111, 0, 0, 131, 30, 44, 136, 132, 32, 44, 137,
        133, 35, 44, 139, 14, 189, 57, 141, 104, 80, 16, 124,
    ];
    assert_eq!(v.as_slice(), expected);
}