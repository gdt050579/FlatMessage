use crate::*;
use flat_message::*;

#[test]
fn check_simple_object() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: String,
        s2: &'a str,
    }
    let t = Test {
        s1: "Hello".to_string(),
        s2: "World",
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&v).unwrap();
    assert_eq!(t, deserialized);
}

#[test]
fn check_string_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: String,
        s2: &'a str,
    }
    let t = Test {
        s1: "Hello".to_string(),
        s2: "World",
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let buf = v.as_slice();
    assert_eq!(
        buf,
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            5, // size of s1 (Hello)
            87, 111, 114, 108, 100, // s1 - "Hello"
            5,   // size of s2 (World)
            72, 101, 108, 108, 111, // s2 - "World"
            14, 180, 81, 5, // hash for s1
            14, 184, 81, 8,  // hash for s2
            8,  // offset for s1 - 8
            14  // offset for s2 - 14
        ]
    );
}

#[test]
fn check_vector_object() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: Vec<String>,
        s2: Vec<&'a str>,
    }
    let t = Test {
        s1: vec!["Hello".to_string(), "World".to_string()],
        s2: vec!["abc", "xyz"],
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&v).unwrap();
    assert_eq!(t, deserialized);
}

#[test]
fn check_serde_string_into_str() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        name: String,
        surname: String,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct2<'a> {
        name: &'a str,
        surname: &'a str,
    }

    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe".to_string(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestStruct2::deserialize_from(&output).unwrap();
    assert_eq!(b.name, a.name.as_str());
    assert_eq!(b.surname, a.surname.as_str());
}

#[test]
fn check_serde_vec_string_and_str_unchecked() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        v1: Vec<String>,
        v2: Vec<&'a str>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        v1: vec![
            "Hello".to_string(),
            "World".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        ],
        v2: vec![
            "Hello", "World", "John", "Doe", "this", "is", "a", "test", "of", "strings", "and",
            "more",
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = unsafe { TestStruct::deserialize_from_unchecked(&v).unwrap() };
    assert_eq!(s.value, ds.value);
    assert_eq!(s.v1, ds.v1);
    assert_eq!(s.v2, ds.v2);
}

#[test]
fn check_serde_vec_string_and_str() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        v1: Vec<String>,
        v2: Vec<&'a str>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        v1: vec![
            "Hello".to_string(),
            "World".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        ],
        v2: vec![
            "Hello", "World", "John", "Doe", "this", "is", "a", "test", "of", "strings", "and",
            "more",
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.v1, ds.v1);
    assert_eq!(s.v2, ds.v2);
}

#[test]
fn check_serde_vec_string() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct {
        value: u32,
        v1: Vec<String>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        v1: vec![
            "Hello".to_string(),
            "World".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.v1, ds.v1);

    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            64, 226, 1, 0, // value - 123456
            4, // size of v1 (4 elements)
            5, 72, 101, 108, 108, 111, // v1[0] - "Hello" (5 bytes)
            5, 87, 111, 114, 108, 100, // v1[1] - "World" (5 bytes)
            4, 74, 111, 104, 110, // v1[2] - "John" (4 bytes)
            3, 68, 111, 101, // v1[3] - "Doe" (3 bytes)
            0, 0, // padding for alignment
            3, 211, 94, 66, // hash for value (123456)
            142, 70, 74, 148, // hash for v1
            8,   // offset for value - 8
            12   // offset for v1 - 12
        ]
    );
}

#[test]
fn check_serde_vec_str() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        value: u32,
        v1: Vec<&'a str>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        v1: vec!["Hello", "World", "John", "Doe"],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.v1, ds.v1);

    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 2, 0, 0, 0, // Header
            64, 226, 1, 0, // value - 123456
            4, // size of v1 (4 elements)
            5, 72, 101, 108, 108, 111, // v1[0] - "Hello" (5 bytes)
            5, 87, 111, 114, 108, 100, // v1[1] - "World" (5 bytes)
            4, 74, 111, 104, 110, // v1[2] - "John" (4 bytes)
            3, 68, 111, 101, // v1[3] - "Doe" (3 bytes)
            0, 0, // padding for alignment
            3, 211, 94, 66, // hash for value (123456)
            142, 70, 74, 148, // hash for v1
            8,   // offset for value - 8
            12   // offset for v1 - 12
        ]
    );
}
