use crate::*;
use flat_message::*;

#[test]
fn check_no_metadata() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let si = StructureInformation::try_from(&output).unwrap();
    assert_eq!(si.version(), None);
    assert_eq!(si.timestamp(), None);
    assert_eq!(si.unique_id(), None);
    assert_eq!(si.name(), Some(name!("TestStruct")));
}

#[test]
fn check_all_metadata() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(version = 5, store_name = true)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
        id: UniqueID,
        timestamp: Timestamp,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
        id: UniqueID::with_value(123456),
        timestamp: Timestamp::with_value(654321),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let si = StructureInformation::try_from(&output).unwrap();
    assert_eq!(si.version(), Some(5));
    assert_eq!(si.timestamp(), Some(654321));
    assert_eq!(si.unique_id(), Some(123456));
    assert_eq!(si.name(), Some(name!("TestStruct")));
}

#[test]
fn check_no_metadata_no_name() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let si = StructureInformation::try_from(&output).unwrap();
    assert_eq!(si.version(), None);
    assert_eq!(si.timestamp(), None);
    assert_eq!(si.unique_id(), None);
    assert_eq!(si.name(), None);
}

#[test]
fn check_unique_id_buffer() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        id: UniqueID,
    }
    let t = Test {
        x: 1,
        id: UniqueID::with_value(123),
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            70, 76, 77, 1, 1, 0, 0, 32, // x
            1, 0, 0, 0, // hash for x
            8, 80, 12, 253, // offset for x
            8,   // unique_id
            123, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn check_unique_id_serde() {
    #[derive(FlatMessage, Eq, PartialEq, Debug)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        id: UniqueID,
    }
    validate_correct_serde(Test {
        x: 1,
        id: UniqueID::with_value(123),
    });
}

#[test]
fn check_timestamp_buffer() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        id: Timestamp,
    }
    let t = Test {
        x: 1,
        id: Timestamp::with_value(123),
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            70, 76, 77, 1, 1, 0, 0, 16, // x
            1, 0, 0, 0, // hash for x
            8, 80, 12, 253, // offset for x
            8,   // timestamp
            123, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn check_timestamp_serde() {
    #[derive(FlatMessage, Eq, PartialEq, Debug)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        t: Timestamp,
    }
    validate_correct_serde(Test {
        x: 1,
        t: Timestamp::with_value(321),
    });
}

#[test]
fn check_timestamp_unique_id_serde() {
    #[derive(FlatMessage, Eq, PartialEq, Debug)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        t: Timestamp,
        i: UniqueID,
    }
    validate_correct_serde(Test {
        x: 1,
        t: Timestamp::with_value(321),
        i: UniqueID::with_value(92),
    });
}
