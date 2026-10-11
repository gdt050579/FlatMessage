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
