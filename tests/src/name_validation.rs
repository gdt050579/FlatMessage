use crate::*;
use flat_message::*;


#[test]
fn check_serde_name_validation() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(validate_name = true)]
    struct TestStruct1 {
        value: u64,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct TestStruct2 {
        value: u64,
    }
    let a_1 = TestStruct1 { value: 12 };
    let a_2 = TestStruct2 { value: 24 };

    let mut output_1 = Storage::default();
    let mut output_2 = Storage::default();
    a_1.serialize_to(&mut output_1, Config::default()).unwrap();
    a_2.serialize_to(&mut output_2, Config::default()).unwrap();

    // from TestStruct1 to TestStruct1
    let b = TestStruct1::deserialize_from(&output_1).unwrap();
    assert_eq!(a_1.value, b.value);

    // from TestStruct1 to TestStruct2 (no validation name required -> should be possible)
    let b = TestStruct2::deserialize_from(&output_1).unwrap();
    assert_eq!(a_1.value, b.value);

    // from TestStruct2 to TestStruct1 (validation name required -> should not be possible)
    let b = TestStruct1::deserialize_from(&output_2);
    assert!(b.is_err());
    assert_eq!(b.err(), Some(flat_message::Error::UnmatchedName));

    // from TestStruct2 to TestStruct2
    let b = TestStruct2::deserialize_from(&output_2).unwrap();
    assert_eq!(a_2.value, b.value);
}

#[test]
fn check_interchangeability_ignoring_name_1() {
    // TestStruct1 has store_name = false and since TestStruct2 has validate_name = false, the name will be ignored
    // as such converting a TestStruct1 to a TestStruct2 should be possible (only "c" field is present in both structs and will be copied from TestStruct1 to TestStruct2)
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct1 {
        a: u8,
        b: u16,
        c: u32
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(validate_name = false)]
    struct TestStruct2 {
        c: u32
    }

    let ts1 = TestStruct1 { a: 1, b: 2, c: 3 };
    let mut s = Storage::default();
    ts1.serialize_to(&mut s, Config::default()).unwrap();
    let ts2 = TestStruct2::deserialize_from(&s).unwrap();
    assert_eq!(ts2.c, 3);
}

#[test]
fn check_interchangeability_ignoring_name_2() {
    // TestStruct1 has store_name = true and since TestStruct2 has validate_name = false, the name will be ignored
    // as such converting a TestStruct1 to a TestStruct2 should be possible (only "c" field is present in both structs and will be copied from TestStruct1 to TestStruct2)
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = true)]
    struct TestStruct1 {
        a: u8,
        b: u16,
        c: u32
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(validate_name = false)]
    struct TestStruct2 {
        c: u32
    }

    let ts1 = TestStruct1 { a: 1, b: 2, c: 3 };
    let mut s = Storage::default();
    ts1.serialize_to(&mut s, Config::default()).unwrap();
    let ts2 = TestStruct2::deserialize_from(&s).unwrap();
    assert_eq!(ts2.c, 3);
}

#[test]
fn check_interchangeability_ignoring_name_3() {
    // TestStruct2 has validate_name = true and since TestStruct1 has store_name = true, the name will be validated
    // in this case the deserialization will fail because the name is not the same
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = true)]
    struct TestStruct1 {
        a: u8,
        b: u16,
        c: u32
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(validate_name = true)]
    struct TestStruct2 {
        c: u32
    }

    let ts1 = TestStruct1 { a: 1, b: 2, c: 3 };
    let mut s = Storage::default();
    ts1.serialize_to(&mut s, Config::default()).unwrap();
    let b = TestStruct2::deserialize_from(&s);
    assert!(b.is_err());
    assert_eq!(b.err(), Some(flat_message::Error::UnmatchedName));
}

#[test]
fn check_interchangeability_ignoring_name_4() {
    // TestStruct2 has validate_name = true and since TestStruct1 has store_name = false, an error will be raised since the name can not be validated
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct1 {
        a: u8,
        b: u16,
        c: u32
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(validate_name = true)]
    struct TestStruct2 {
        c: u32
    }

    let ts1 = TestStruct1 { a: 1, b: 2, c: 3 };
    let mut s = Storage::default();
    ts1.serialize_to(&mut s, Config::default()).unwrap();
    let b = TestStruct2::deserialize_from(&s);
    assert!(b.is_err());
    assert_eq!(b.err(), Some(flat_message::Error::NameNotStored));
}


#[test]
fn check_structure_information_with_match() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct TestStruct {
        a: u64,
    }
    let a = TestStruct { a: 12 };

    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let si = StructureInformation::try_from(&output).unwrap();
    assert_eq!(si.timestamp(), None);
    assert_eq!(si.unique_id(), None);
    assert_eq!(si.version(), None);
    if let Some(name) = si.name() {
        match name {
            name!("TestStruct") => {}
            name!("TestStruct2") => panic!("Invalid name"),
            _ => panic!("Invalid name"),
        }
    }
}
