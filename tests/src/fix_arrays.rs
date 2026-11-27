use crate::*;
use flat_message::*;

#[test]
fn check_simple_array() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
    }
    validate_correct_serde(Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    });
}

#[test]
fn check_simple_array_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
    }
    let t = Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0,  // header
            10, // v1 - size of the array
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // v1 - values
            0,  // padding
            26, 70, 74, 148, // hash for v1 (26 = type for FixArray)
            8    // offset of v1
        ]
    );
}

#[test]
fn check_simple_array_muliple_items() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
        v2: [u8; 2],
        v3: [u8; 4],
        v4: [u8; 7],
    }
    validate_correct_serde(Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        v2: [11, 12],
        v3: [13, 14, 15, 16],
        v4: [17, 18, 19, 20, 21, 22, 23],
    });
}

#[test]
fn check_simple_array_muliple_items_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
        v2: [u8; 2],
        v3: [u8; 4],
        v4: [u8; 7],
    }
    let t = Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        v2: [11, 12],
        v3: [13, 14, 15, 16],
        v4: [17, 18, 19, 20, 21, 22, 23],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 4, 0, 0, 0,  // header
            10, // v1 - size of the array
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // v1 - values
            4,  // v3 - size of the array
            13, 14, 15, 16, // v3 - values
            2,  // v2 - size of the array
            11, 12, // v2 - values
            7,  // v4 - size of the array
            17, 18, 19, 20, 21, 22, 23, // v4 - values
            0,  // padding
            26, 70, 74, 148, // hash for v1 (26 = type for FixArray)
            26, 73, 74, 150, // hash for v3 (26 = type for FixArray)
            26, 75, 74, 151, // hash for v2 (26 = type for FixArray)
            26, 78, 74, 153, // hash for v4 (26 = type for FixArray)
            8,   // offset of v1
            19,  // offset of v3
            24,  // offset of v2
            27,  // offset of v4
        ]
    );
}

#[test]
fn check_simple_slice() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: &'a [[u8; 3]],
    }
    let t = Test {
        v1: &[[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let d = Test::deserialize_from(&s).unwrap();
    assert_eq!(d, t);
}

#[test]
fn check_simple_slice_unchecked() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: &'a [[u8; 3]],
    }
    let t = Test {
        v1: &[[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let d = unsafe { Test::deserialize_from_unchecked(&s).unwrap() };
    assert_eq!(d, t);
}

#[test]
fn check_simple_slice_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: &'a [[u8; 3]],
    }
    let t = Test {
        v1: &[[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // header
            3, // v1 - number of fix elements in the array - 3
            2, // v1 - size of the array - 2
            1, 2, 3, // v1[0] - values
            10, 20, 30, // v1[1] - values
            154, 70, 74, 148, // hash for v1 (154 = type for FixArray + slice bit)
            8    // offset of v1 - 8
        ]
    );
}

#[test]
fn check_simple_vec() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Vec<[u8; 3]>,
    }
    let t = Test {
        v1: vec![[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let d = Test::deserialize_from(&s).unwrap();
    assert_eq!(d, t);
}

#[test]
fn check_simple_vec_unchecked() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Vec<[u8; 3]>,
    }
    let t = Test {
        v1: vec![[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let d = unsafe { Test::deserialize_from_unchecked(&s).unwrap() };
    assert_eq!(d, t);
}

#[test]
fn check_simple_vec_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Vec<[u8; 3]>,
    }
    let t = Test {
        v1: vec![[1, 2, 3], [10, 20, 30]],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // header
            3, // v1 - number of fix elements in the array - 3
            2, // v1 - size of the array - 2
            1, 2, 3, // v1[0] - values
            10, 20, 30, // v1[1] - values
            154, 70, 74, 148, // hash for v1 (154 = type for FixArray + slice bit)
            8    // offset of v1 - 8
        ]
    );
}

#[test]
fn check_simple_array_ref() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: &'a [u8; 10],
    }
    let t = Test {
        v1: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let d = Test::deserialize_from(&s).unwrap();
    assert_eq!(d, t);
}

#[test]
fn check_simple_array_ref_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: &'a [u8; 10],
    }
    let t = Test {
        v1: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0,  // header
            10, // v1 - size of the array
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // v1 - values
            0,  // padding
            26, 70, 74, 148, // hash for v1 (26 = type for FixArray)
            8    // offset of v1
        ]
    );
}

#[test]
fn check_array_from_buffer_bounds() {
    // Truncated: length prefix matches N=4, but fewer than 4 bytes remain
    assert_eq!(<[u8; 4] as SerDe>::from_buffer(&[4], 0), None);
    assert_eq!(<[u8; 4] as SerDe>::from_buffer(&[4, 1, 2, 3], 0), None);
    assert_eq!(<[u8; 4] as SerDe>::from_buffer(&[0, 0, 4, 1, 2, 3], 2), None);

    assert_eq!(<&[u8; 4] as SerDe>::from_buffer(&[4], 0), None);
    assert_eq!(<&[u8; 4] as SerDe>::from_buffer(&[4, 1, 2, 3], 0), None);
    assert_eq!(<&[u8; 4] as SerDe>::from_buffer(&[0, 0, 4, 1, 2, 3], 2), None);

    // Count mismatch even when enough bytes exist
    assert_eq!(<[u8; 4] as SerDe>::from_buffer(&[3, 1, 2, 3, 4], 0), None);
    assert_eq!(<&[u8; 4] as SerDe>::from_buffer(&[3, 1, 2, 3, 4], 0), None);

    // Exact boundary succeeds
    assert_eq!(
        <[u8; 4] as SerDe>::from_buffer(&[4, 1, 2, 3, 4], 0),
        Some([1, 2, 3, 4])
    );
    assert_eq!(
        <&[u8; 4] as SerDe>::from_buffer(&[4, 1, 2, 3, 4], 0),
        Some(&[1, 2, 3, 4])
    );

    // Extended length prefix (U8withExtension: 0xFE + u16 little-endian)
    let mut buf = vec![0xFE];
    buf.extend_from_slice(&260u16.to_le_bytes());
    buf.extend(std::iter::repeat_n(0xABu8, 259));
    assert_eq!(<[u8; 260] as SerDe>::from_buffer(&buf, 0), None);
    assert_eq!(<&[u8; 260] as SerDe>::from_buffer(&buf, 0), None);

    buf.push(0xAB);
    assert_eq!(<[u8; 260] as SerDe>::from_buffer(&buf, 0), Some([0xAB; 260]));
    assert_eq!(
        <&[u8; 260] as SerDe>::from_buffer(&buf, 0),
        Some(&[0xAB; 260])
    );
}

#[test]
fn check_array_slice_from_buffer_bounds_and_overflow() {
    // Truncated slice payload: N=4, count=2, only 7 bytes of payload
    assert_eq!(
        <[u8; 4] as SerDeSlice>::from_buffer(&[4, 2, 1, 2, 3, 4, 5, 6, 7], 0),
        None
    );

    // Element size mismatch
    assert_eq!(
        <[u8; 4] as SerDeSlice>::from_buffer(&[3, 2, 1, 2, 3, 4, 5, 6], 0),
        None
    );

    // Exact slice payload succeeds
    assert_eq!(
        <[u8; 4] as SerDeSlice>::from_buffer(&[4, 2, 1, 2, 3, 4, 5, 6, 7, 8], 0),
        Some(&[[1, 2, 3, 4], [5, 6, 7, 8]][..])
    );

    // Huge count (u32::MAX) should return None without overflowing or panicking
    let huge_count = [4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    assert_eq!(<[u8; 4] as SerDeSlice>::from_buffer(&huge_count, 0), None);

    // Large N (0x1_0000) with large count (0x1_0000) overflows usize on 32-bit and exceeds buf.len() on 64-bit
    let mut large_n_and_count = vec![0xFF];
    large_n_and_count.extend_from_slice(&0x1_0000u32.to_le_bytes());
    large_n_and_count.push(0xFF);
    large_n_and_count.extend_from_slice(&0x1_0000u32.to_le_bytes());
    assert_eq!(
        <[u8; 0x1_0000] as SerDeSlice>::from_buffer(&large_n_and_count, 0),
        None
    );
}
