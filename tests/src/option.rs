use crate::*;
use flat_message::*;

#[test]
fn check_none() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    validate_correct_serde(Test { v1: None, v2: None });
}

#[test]
fn check_some() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    validate_correct_serde(Test {
        v1: Some(1),
        v2: Some(2),
    });
}

#[test]
fn check_none_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    let t = Test { v1: None, v2: None };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            8, 70, 74, 148, // hash for v1
            3, 75, 74, 151, // hash for v2
            0,   // offset of v1 (0 = None (invalid offset))
            0    // offset of v2 (0 = None (invalid offset))
        ]
    );
}

#[test]
fn check_some_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    let t = Test {
        v1: Some(1),
        v2: Some(2),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            1, 0, 0, 0, // v1 value (1)
            2, 0, 0, 0, // v2 value (2)
            8, 70, 74, 148, // hash for v1
            3, 75, 74, 151, // hash for v2
            8,   // offset of v1
            12   // offset of v2
        ]
    );
}

#[test]
fn check_some_none_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    let t = Test {
        v1: Some(1),
        v2: None,
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            1, 0, 0, 0, // v1 value (1)
            8, 70, 74, 148, // hash for v1
            3, 75, 74, 151, // hash for v2
            8,   // offset of v1
            0    // offset of v2 (0 = None)
        ]
    );
}

#[test]
fn check_vec() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<Vec<u32>>,
        v2: Option<Vec<bool>>,
    }
    validate_correct_serde(Test {
        v1: Some([1, 2, 3, 4].to_vec()),
        v2: None,
    });
}

#[test]
fn check_vec_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<Vec<u8>>,
        v2: Option<Vec<String>>,
        v3: Option<bool>,
    }
    let t = Test {
        v1: Some([1, 2, 3, 4].to_vec()),
        v2: Some(["Hello".to_string(), "xyz".to_string()].to_vec()),
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
fn check_option_to_non_option() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestOpt {
        v1: Option<u32>,
        v2: Option<u32>,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestNoOpt {
        v1: u32,
        v2: u32,
    }
    let t_opt = TestOpt {
        v1: Some(1),
        v2: Some(2),
    };
    let mut s = Storage::default();
    t_opt.serialize_to(&mut s, Config::default()).unwrap();
    let t_no_opt = TestNoOpt::deserialize_from(&s).unwrap();
    assert_eq!(t_no_opt.v1, 1);
    assert_eq!(t_no_opt.v2, 2);
}

#[test]
fn check_option_to_non_option_error() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestOpt {
        v1: Option<u32>,
        v2: Option<u32>,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestNoOpt {
        v1: u32,
        v2: u32,
    }
    let t_opt = TestOpt {
        v1: Some(1),
        v2: None,
    };
    let mut s = Storage::default();
    t_opt.serialize_to(&mut s, Config::default()).unwrap();
    let res = TestNoOpt::deserialize_from(&s);
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        flat_message::Error::InvalidFieldOffset((0, 12))
    );
}

#[test]
fn check_option_to_non_option_error_unchecked() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestOpt {
        v1: Option<u32>,
        v2: Option<u32>,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct TestNoOpt {
        v1: u32,
        v2: u32,
    }
    let t_opt = TestOpt {
        v1: Some(1),
        v2: None,
    };
    let mut s = Storage::default();
    t_opt.serialize_to(&mut s, Config::default()).unwrap();
    let res = unsafe { TestNoOpt::deserialize_from_unchecked(&s) };
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        flat_message::Error::InvalidFieldOffset((0, 12))
    );
}

#[test]
fn check_slice() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        v1: Option<&'a [u32]>,
        v2: Option<Vec<bool>>,
    }
    let t = Test {
        v1: Some(&[1, 2, 3, 4]),
        v2: None,
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_struct_some() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq, Default)]
    struct Configuration {
        timeout: u32,
        retries: u8,
    }

    #[derive(Debug, FlatMessage, Eq, PartialEq, Default)]
    #[flat_message_options(store_name: false)]
    struct Request {
        #[flat_message_item(align = 4, kind = struct)]
        config: Option<Configuration>,
    }
    let t = Request {
        config: Some(Configuration {
            timeout: 1000,
            retries: 3,
        }),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Request::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_struct_none() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq, Default)]
    struct Configuration {
        timeout: u32,
        retries: u8,
    }

    #[derive(Debug, FlatMessage, Eq, PartialEq)]
    #[flat_message_options(store_name: false)]
    struct Request {
        #[flat_message_item(align = 4, kind = struct, validate = strict)]
        config: Option<Configuration>,
    }
    let t = Request { config: None };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Request::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_struct_none_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq, Default)]
    struct Configuration {
        timeout: u32,
        retries: u8,
    }

    #[derive(Debug, FlatMessage, Eq, PartialEq, Default)]
    #[flat_message_options(store_name: false)]
    struct Request {
        #[flat_message_item(align = 4, kind = struct, validate: strict)]
        config: Option<Configuration>,
    }
    let t = Request { config: None };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            32, 102, 255, 35, // hash for config
            0,  // offset of config (0 = None)
        ]
    );
}
