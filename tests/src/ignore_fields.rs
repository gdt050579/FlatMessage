use crate::*;
use flat_message::*;
use std::marker::PhantomData;

#[test]
fn check_phanton_data_serde() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: PhantomData<u32>,
    }
    validate_correct_serde(Test {
        x: 12,
        y: PhantomData,
    });
}

#[test]
fn check_phanton_data_missing() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test1 {
        x: u8,
        y: PhantomData<u32>,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test2 {
        x: u8,
    }
    let mut v1 = Storage::default();
    let t1 = Test1 {
        x: 12,
        y: PhantomData,
    };
    t1.serialize_to(&mut v1, Config::default()).unwrap();
    let mut v2 = Storage::default();
    let t2 = Test2 { x: 12 };
    t2.serialize_to(&mut v2, Config::default()).unwrap();
    assert_eq!(v1.as_slice(), v2.as_slice(),);
}

#[test]
fn check_skip_with_default() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(ignore = true)]
        y: u32,
    }
    let data = Test { x: 1, y: 2 };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap(); // y is skipped
    let ds = Test::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.y, 0); // not 2 -> 0 is the default for y
}

#[test]
fn check_skip_with_custom_default() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(ignore = true, default = 10)]
        y: u32,
    }
    let data = Test { x: 1, y: 2 };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap(); // y is skipped
    let ds = Test::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.y, 10); // not 2 -> 10 is the custom default for y
}

#[test]
fn check_skip_repr() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(ignore = true)]
        _y: u32,
    }
    let data = Test { x: 1, _y: 2 };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap(); // y is skipped
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // header - only one variable
            1, // value of x
            0, 0, 0, // padding
            1, 80, 12, 253, // hash for "x"
            8    // offset for "x"
        ]
    );
}

#[test]
fn check_skip_for_packed_struct_with_default() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct TestPacked {
        x: u8,
        #[flat_message_item(ignore = true)]
        y: u32,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct Test {
        x: u8,
        #[flat_message_item(kind = packed, align = 1)]
        t: TestPacked,
    }
    let data = Test {
        x: 1,
        t: TestPacked { x: 2, y: 3 },
    };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap(); // y is skipped
    let ds = Test::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.t.x, 2);
    assert_eq!(ds.t.y, 0); // not 3 -> 0 is the default for y
}

#[test]
fn check_skip_for_packed_struct_with_custom_default() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct TestPacked {
        x: u8,
        #[flat_message_item(ignore = true, default = 10)]
        y: u32,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct Test {
        x: u8,
        #[flat_message_item(kind = packed, align = 1)]
        t: TestPacked,
    }
    let data = Test {
        x: 1,
        t: TestPacked { x: 2, y: 3 },
    };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap(); // y is skipped
    let ds = Test::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.t.x, 2);
    assert_eq!(ds.t.y, 10); // not 3 -> 10 is the custom default for y
}
