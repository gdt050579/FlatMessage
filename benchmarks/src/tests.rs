use flat_message::*;

macro_rules! check_field_value {
    ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
        let val: $type = $flat_message_buffer.get($field_name).unwrap();
        assert_eq!(val, $value);
    };
}
macro_rules! check_field_value_unsafe {
    ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
        let val: $type = unsafe { $flat_message_buffer.get_unchecked($field_name).unwrap() };
        assert_eq!(val, $value);
    };
}

#[test]
fn check_flat_message_buffer_one_field_i32() {
    #[flat_message]
    struct TestStruct {
        my_field: i32,
    }
    let a = TestStruct {
        my_field: 123456,
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value!(name!("my_field"), i32, 123456, buf);
}

#[test]
fn check_flat_message_buffer_one_field_str() {
    #[flat_message]
    struct TestStruct {
        my_field: String,
    }
    let a = TestStruct {
        my_field: "Hello, World!".to_string(),
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value!(name!("my_field"), &str, "Hello, World!", buf);
}

#[test]
fn check_flat_message_buffer_two_fields_i32_i8() {
    #[flat_message]
    struct TestStruct {
        size: i32,
        dimension: i8,
    }
    let a = TestStruct {
        size: -12345,
        dimension: -100,
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value!(name!("size"), i32, -12345, buf);
    check_field_value!(name!("dimension"), i8, -100, buf);
}

#[test]
fn check_flat_message_buffer_two_fields_string_string() {
    #[flat_message]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value!(name!("name"), &str, "John", buf);
    check_field_value!(name!("surname"), &str, "Doe", buf);
}

#[test]
fn check_flat_message_buffer_safe() {
    #[flat_message]
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
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value!(name!("name"), &str, "John", buf);
    check_field_value!(name!("surname"), &str, "Doe", buf);
    check_field_value!(name!("math"), u8, 100, buf);
    check_field_value!(name!("engligh"), u8, 90, buf);
    check_field_value!(name!("passed"), bool, true, buf);
    check_field_value!(name!("average"), f64, 95.0, buf);
}

#[test]
fn check_flat_message_buffer_unsafe() {
    #[flat_message]
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
        metadata: MetaData::default(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    check_field_value_unsafe!(name!("name"), &str, "John", buf);
    check_field_value_unsafe!(name!("surname"), &str, "Doe", buf);
    check_field_value_unsafe!(name!("math"), u8, 100, buf);
    check_field_value_unsafe!(name!("engligh"), u8, 90, buf);
    check_field_value_unsafe!(name!("passed"), bool, true, buf);
    check_field_value_unsafe!(name!("average"), f64, 95.0, buf);
}

#[test]
fn check_flat_message_metadata() {
    #[flat_message(version = 5)]
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
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    let metadata = buf.metadata();
    assert_eq!(buf.version(), Some(5));
    assert_eq!(metadata.timestamp(), Some(123456));
    assert_eq!(metadata.unique_id(), Some(654321));
    assert_eq!(buf.name(), Some(name!("TestStruct")));
}

#[test]
fn check_flat_message_no_metadata() {
    #[flat_message(metadata: false)]
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
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    let metadata = buf.metadata();
    assert_eq!(buf.version(), None);
    assert_eq!(metadata.timestamp(), None);
    assert_eq!(metadata.unique_id(), None);
    assert_eq!(buf.name(), Some(name!("TestStruct")));
}

#[test]
fn check_flat_message_no_metadata_no_name() {
    #[flat_message(store_name: false, metadata: false)]
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
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    let metadata = buf.metadata();
    assert_eq!(buf.version(), None);
    assert_eq!(metadata.timestamp(), None);
    assert_eq!(metadata.unique_id(), None);
    assert_eq!(buf.name(), None);
}

#[test]
fn check_serde_full() {
    #[flat_message]
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
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestStruct::deserialize_from(&output).unwrap();
    assert_eq!(a.name, b.name);
    assert_eq!(a.surname, b.surname);
    assert_eq!(a.math, b.math);
    assert_eq!(a.engligh, b.engligh);
    assert_eq!(a.passed, b.passed);
    assert_eq!(a.average, b.average);
    assert_eq!(a.metadata.timestamp(), b.metadata.timestamp());
    assert_eq!(a.metadata.unique_id(), b.metadata.unique_id());
}

#[test]
fn check_serde_into_smaller_struct() {
    #[flat_message]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[flat_message(metadata: false)]
    struct TestSmallerStruct {
        name: String,
        math: u8,
        engligh: u8,
        average: f64,
    }

    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestSmallerStruct::deserialize_from(&output).unwrap();
    assert_eq!(a.name, b.name);
    assert_eq!(a.math, b.math);
    assert_eq!(a.engligh, b.engligh);
    assert_eq!(a.average, b.average);
}

#[test]
fn check_serde_into_different_struct() {
    #[flat_message]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[flat_message(metadata: false)]
    struct TestSmallerStruct {
        a: u8,
        b: u16,
        math: u16,
    }

    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestSmallerStruct::deserialize_from(&output);
    assert_eq!(b.is_err(), true);
}

#[test]
fn check_serde_into_different_type() {
    #[flat_message]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[flat_message]
    struct TestStruct2<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u16, // english is not the same type
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
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestStruct2::deserialize_from(&output);
    assert_eq!(b.is_err(), true);
}

#[test]
fn check_serde_string_into_str() {
    #[flat_message(metadata: false)]
    struct TestStruct {
        name: String,
        surname: String,
    }

    #[flat_message(metadata: false)]
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
    // the following lines should not compile
    // output.clear();
    // output.resize(0xFFFF,b'a');
    assert_eq!(b.name, a.name.as_str());
    assert_eq!(b.surname, a.surname.as_str());
}

#[test]
fn check_serde_full_unchecked() {
    #[flat_message]
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
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = unsafe { TestStruct::deserialize_from_unchecked(&output).unwrap() };
    assert_eq!(a.name, b.name);
    assert_eq!(a.surname, b.surname);
    assert_eq!(a.math, b.math);
    assert_eq!(a.engligh, b.engligh);
    assert_eq!(a.passed, b.passed);
    assert_eq!(a.average, b.average);
    assert_eq!(a.metadata.timestamp(), b.metadata.timestamp());
    assert_eq!(a.metadata.unique_id(), b.metadata.unique_id());
}

#[test]
fn check_structure_information() {
    #[flat_message(version: 12)]
    struct TestStruct {
        a: u64,
        b: u32,
    }
    let a = TestStruct {
        a: 12,
        b: 34,
        metadata: MetaDataBuilder::new()
            .timestamp(123456)
            .unique_id(654321)
            .build(),
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let si = StructureInformation::try_from(&output).unwrap();
    assert_eq!(si.timestamp(), Some(123456));
    assert_eq!(si.unique_id(), Some(654321));
    assert_eq!(si.version(), Some(12));
    assert_eq!(si.name(), Some(name!("TestStruct")));
}

#[test]
fn check_structure_information_with_match() {
    #[flat_message(metadata: false)]
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

#[test]
fn check_serde_name_validation() {
    #[flat_message(metadata: false, validate_name: true)]
    struct TestStruct1 {
        value: u64,
    }
    #[flat_message(metadata: false)]
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
    assert_eq!(b.is_err(), true);

    // from TestStruct2 to TestStruct2
    let b = TestStruct2::deserialize_from(&output_2).unwrap();
    assert_eq!(a_2.value, b.value);
}

#[test]
fn check_serde_version_compatibility_check() {
    mod v1 {
        use flat_message::*;
        #[flat_message(version: 1, compatible_versions: "1")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    mod v2 {
        use flat_message::*;
        #[flat_message(version: 2, compatible_versions: "1,2")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    mod v3 {
        use flat_message::*;
        #[flat_message(version: 3, compatible_versions: "<3")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    let mut o1 = Storage::default();
    let mut o2 = Storage::default();
    let mut o3 = Storage::default();
    v3::TestStruct {
        value: 3,
        metadata: MetaDataBuilder::new().timestamp(333).unique_id(33).build(),
    }
    .serialize_to(&mut o3, Config::default())
    .unwrap();
    v2::TestStruct {
        value: 2,
        metadata: MetaDataBuilder::new().timestamp(222).unique_id(22).build(),
    }
    .serialize_to(&mut o2, Config::default())
    .unwrap();
    v1::TestStruct {
        value: 1,
        metadata: MetaDataBuilder::new().timestamp(111).unique_id(11).build(),
    }
    .serialize_to(&mut o1, Config::default())
    .unwrap();
    let v1_from_v3 = v1::TestStruct::deserialize_from(&o3);
    let v1_from_v2 = v1::TestStruct::deserialize_from(&o2);
    let v2_from_v3 = v2::TestStruct::deserialize_from(&o3);
    let v3_from_v1 = v3::TestStruct::deserialize_from(&o1);
    let v3_from_v2 = v3::TestStruct::deserialize_from(&o2);
    let v2_from_v1 = v2::TestStruct::deserialize_from(&o1);
    assert_eq!(
        v1_from_v2.err(),
        Some(flat_message::Error::IncompatibleVersion(2))
    );
    assert_eq!(
        v1_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(
        v2_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(v3_from_v1.unwrap().value, 1);
    assert_eq!(v3_from_v2.unwrap().value, 2);
    assert_eq!(v2_from_v1.unwrap().value, 1);
}

#[test]
fn check_derive() {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[flat_message]
    struct TestStruct {
        a: i32,
        b: bool,
        c: u16,
    }
    let v1 = TestStruct {
        a: 1,
        b: true,
        c: 123,
        metadata: MetaDataBuilder::new().timestamp(1).unique_id(2).build(),
    };
    let v2 = v1;
    assert_eq!(v1.a, v2.a);
    assert_eq!(v1.b, v2.b);
    assert_eq!(v1.c, v2.c);
    assert_eq!(v1.metadata, v2.metadata);
    assert_eq!(v1, v2);
    let mut storage = Storage::default();
    v1.serialize_to(&mut storage, Config::default()).unwrap();
    let v3 = TestStruct::deserialize_from(&storage).unwrap();
    assert_eq!(v1, v3);
}

#[test]
fn check_clone() {
    #[flat_message]
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestStruct {
        a: String,
        b: String,
    }
    let v1 = TestStruct {
        a: "Hello".to_string(),
        b: "World".to_string(),
        metadata: MetaDataBuilder::new().timestamp(1).unique_id(2).build(),
    };
    let v2 = v1.clone();
    assert_eq!(v1.a, v2.a);
    assert_eq!(v1.b, v2.b);
    assert_eq!(v1.metadata, v2.metadata);
    assert_eq!(v1, v2);
    let mut storage = Storage::default();
    v1.serialize_to(&mut storage, Config::default()).unwrap();
    let v3 = TestStruct::deserialize_from(&storage).unwrap();
    assert_eq!(v1, v3);
}

#[test]
fn check_serialization_checksum() {
    #[flat_message(checksum: true, store_name: false, metadata: false)]
    struct TestStruct1 {
        value: u32,
    }
    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct2 {
        value: u32,
    }
    let v1 = TestStruct1 { value: 123456 };
    let v2 = TestStruct2 { value: 123456 };
    let mut storage = Storage::default();
    v1.serialize_to(&mut storage, Config::default()).unwrap();
    let expected_output = vec![
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ];
    assert_eq!(storage.as_slice(), expected_output);
    let len_v1 = storage.len();
    v2.serialize_to(&mut storage, Config::default()).unwrap();
    let expected_output = vec![71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8];
    assert_eq!(storage.as_slice(), expected_output);
    let len_v2 = storage.len();
    // TestStruct1 has 4 bytes more than TestStruct2 (for the checksum)
    assert_eq!(len_v1, len_v2 + 4);
}

#[test]
fn check_serde_with_checksum() {
    #[flat_message(checksum: true, store_name: false, metadata: false)]
    struct TestStruct<'a> {
        value: u32,
        b: bool,
        name: String,
        surname: &'a str,
        age: i32,
    }
    let s = TestStruct {
        value: 123456,
        b: true,
        name: "John".to_string(),
        surname: "Doe",
        age: 30,
    };
    let mut storage = Storage::default();
    s.serialize_to(&mut storage, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&storage).unwrap();
    assert_eq!(s.age, ds.age);
    assert_eq!(s.b, ds.b);
    assert_eq!(s.value, ds.value);
    assert_eq!(s.name, ds.name);
    assert_eq!(s.surname, ds.surname);
}

#[test]
fn check_deserialization_checksum_always() {
    #[flat_message(checksum: true, store_name: false, metadata: false, validate_checksum: always)]
    struct TestStruct {
        value: u32,
    }
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 255, 255, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer);
    match v.err() {
        Some(flat_message::Error::InvalidChecksum(_)) => {}
        _ => panic!("Invalid error - expected InvalidChecksum"),
    }
}

#[test]
fn check_deserialization_checksum_auto() {
    #[flat_message(checksum: true, store_name: false, metadata: false)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // invalid checksum
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 255, 255, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer);
    match v.err() {
        Some(flat_message::Error::InvalidChecksum(_)) => {}
        _ => panic!("Invalid error - expected InvalidChecksum"),
    }
    // checksum is missing
    let buffer =
        Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
}

#[test]
fn check_deserialization_checksum_ignore() {
    #[flat_message(checksum: true, store_name: false, metadata: false, validate_checksum: ignore)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // invalid checksum (deserialization should still happen)
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // checksum is missing
    let buffer =
        Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
}

#[test]
fn check_deserialization_checksum_unchecked_always() {
    #[flat_message(checksum: true, store_name: false, metadata: false, validate_checksum: always)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
    // invalid checksum (deserialization should still happen)
    let buffer = Storage::from_buffer(&[
        71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
    ]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
    // checksum is missing (deserialization should still happen)
    let buffer =
        Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
}

#[test]
fn check_max_size_for_serialization() {
    #[flat_message]
    struct TestStruct {
        value: u32,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123456,
        metadata: MetaData::default(),
    };
    let result = s.serialize_to(&mut v, Config::default());
    assert!(result.is_ok());
    let result = s.serialize_to(&mut v, ConfigBuilder::new().max_size(4).build());
    assert!(result.is_err());
    match result.err() {
        Some(flat_message::Error::ExceedMaxSize(_)) => {}
        _ => panic!("Invalid error - expected MaxSizeExceeded"),
    }
}

#[test]
fn check_serde_buffer_i8() {
    #[flat_message(metadata: false)]
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
fn check_serde_buffer_u8() {
    #[flat_message(metadata: false)]
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
fn check_buffer_format_u16() {
    #[flat_message(metadata: false,store_name: false)]
    struct TestStruct {
        b2: Vec<u16>,
    }
    let mut v = Vec::new();
    let s = TestStruct {
        b2: [1, 2, 3].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v,
        vec![71, 84, 72, 1, 1, 0, 0, 0, 3, 0, 1, 0, 2, 0, 3, 0, 130, 41, 44, 143, 8]
    );
}

#[test]
fn check_serde_buffer_u16() {
    #[flat_message(metadata: false, store_name: false)]
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
fn check_serde_buffer_i16() {
    #[flat_message(metadata: false, store_name: false)]
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
fn check_serde_buffer_32bit_integer() {
    #[flat_message(metadata: false, store_name: false)]
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
fn check_aliganemnt_order_u32_u16_string() {
    #[flat_message(metadata: false, store_name: false)]
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
    let expected = vec![
        71u8, 84, 72, 1, 3, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 3,
        0, 1, 0, 2, 0, 3, 0, 5, 72, 101, 108, 108, 111, 0, 0, 14, 159, 54, 27, 131, 216, 51, 208,
        130, 226, 119, 250, 36, 8, 28,
    ];
    assert_eq!(v.as_slice(), expected.as_slice());
}

#[test]
fn check_serde_buffer_float_32() {
    #[flat_message(metadata: false, store_name: false)]
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
    #[flat_message(metadata: false, store_name: false)]
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
        b3: &[-1, 2, -3, 0x123456_7890, -6, 7, -8, i64::MIN, -10, i64::MAX],
        b4: [1, -2, 300, 0x123456_7890, -678910876, i64::MIN, i64::MAX].to_vec(),
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
fn check_serde_128_bits_alignament() {
    #[flat_message(metadata: false, store_name: false)]
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
        71, 84, 72, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 10,
        0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0,
        0, 0, 43, 0, 0, 0, 5, 72, 101, 108, 108, 111, 0, 0, 131, 30, 44, 136, 132, 32, 44, 137,
        133, 35, 44, 139, 14, 189, 57, 141, 104, 80, 16, 124,
    ];
    assert_eq!(v.as_slice(), expected);
}

#[test]
fn check_serde_128_bits_buffers() {
    #[flat_message(metadata: false, store_name: false)]
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
            0x123456_7890,
            -6,
            7,
            -8,
            i128::MIN,
            -10,
            i128::MAX,
        ],
        b4: [1, -2, 300, 0x123456_7890, -678910876, i128::MIN, i128::MAX].to_vec(),
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
fn check_enum() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct {
        value: u8,
        #[flat_message(repr = u8, kind = enum)]
        color: Color,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            71, 84, 72, 1, 2, 0, 0, 0, 237, 103, 151, 167, 10, 123, 0, 0, 19, 98, 126, 61, 1, 211,
            94, 66, 8, 13
        ]
    );
}

#[test]
fn check_enum_add_variant() {
    mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }
    mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellor = 200,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }

    let mut v = Storage::default();
    let s = v1::TestStruct {
        value: 123,
        color: v1::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = v2::TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color as u8, ds.color as u8);
}

#[test]
fn check_enum_add_variant_sealed() {
    mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        #[sealed]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }
    mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellor = 200,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }

    let mut v = Storage::default();
    let s = v1::TestStruct {
        value: 123,
        color: v1::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = v2::TestStruct::deserialize_from(&v);
    match ds {
        Err(flat_message::Error::FailToDeserialize(_)) => {}
        _ => panic!("Invalid error - expected InvalidEnumVariant"),
    }
}

#[test]
fn check_enum_add_variant_sealed_unchecked() {
    mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        #[sealed]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }
    mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellor = 200,
        }

        #[flat_message(metadata: false, store_name: false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message(repr = u8, kind = enum)]
            pub color: Color,
        }
    }

    let mut v = Storage::default();
    let s = v1::TestStruct {
        value: 123,
        color: v1::Color::Green,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    // seald argument is not checked
    let ds = unsafe { v2::TestStruct::deserialize_from_unchecked(&v).unwrap() };
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color as u8, ds.color as u8);
}

#[test]
fn check_enum_slice_u8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u8, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            71, 84, 72, 1, 2, 0, 0, 0, // TestStruct::color
            // Hash for Color
            237, 103, 151, 167, // number of elements in TestStruct::color
            5, // u8 value for TestStruct::color
            10, 100, 1, 10, 100, // value of TestStruct::value
            123, // alignament padding (to 4 bytes)
            0, // Hash for color
            147, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 18
        ]
    );
}

#[test]
fn check_enum_slice_i8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i8)]
    enum Color {
        Red = 1,
        Green = -10,
        Blue = -100,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = i8, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u16bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u16)]
    enum Color {
        Red = 1234,
        Green = 12345,
        Blue = 2,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u16, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            71, 84, 72, 1, 2, 0, 0, 0, // Hash for Color
            237, 103, 151, 167, // number of items in TestStruct::color (u16)
            5, 0, // 5 items of TestStruct::color (u16) each
            57, 48, 2, 0, 210, 4, 57, 48, 2, 0, // TestStruct::value
            123, // alignament padding (to 4 bytes)
            0, 0, 0, // Hash for color
            148, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 24
        ]
    );
}

#[test]
fn check_enum_slice_i16bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i16)]
    enum Color {
        Red = 1234,
        Green = -12345,
        Blue = 2,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = i16, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u32bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Color {
        Red = 0xFF00FF00,
        Green = 0x00FF00FF,
        Blue = 0xFEFEFEFE,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u32, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_i32bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i32)]
    enum Color {
        Red = -12345678,
        Green = 1,
        Blue = 12345678,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = i32, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_u64bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u64)]
    enum Color {
        Red = 0xFF00FF0012345678,
        Green = 0x00FF00FF11223344,
        Blue = 0xFEFEFEFE99887766,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u64, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_slice_i64bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i64)]
    enum Color {
        Red = 0xFF00FF00123876,
        Green = -11111111111111111,
        Blue = -87614876518,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = i64, kind = enum)]
        color: &'a [Color],
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: &[
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ],
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
}

#[test]
fn check_enum_vec_u8bits() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct {
        value: u8,
        #[flat_message(repr = u8, kind = enum)]
        color: Vec<Color>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        color: [
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ]
        .to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            71, 84, 72, 1, 2, 0, 0, 0, // TestStruct::color
            // Hash for Color
            237, 103, 151, 167, // number of elements in TestStruct::color
            5, // u8 value for TestStruct::color
            10, 100, 1, 10, 100, // value of TestStruct::value
            123, // alignament padding (to 4 bytes)
            0, // Hash for color
            147, 98, 126, 61, // Hash for value
            1, 211, 94, 66, // Offsets
            8, 18
        ]
    );
}

#[test]
fn check_enum_vec_and_slice() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u8, kind = enum)]
        color: Vec<Color>,
        #[flat_message(repr = u8, kind = enum)]
        s_color: &'a [Color],
        #[flat_message(repr = u32, kind = enum)]
        metric: Vec<Metric>,
    }
    let mut v = Storage::default();
    let large_vec = [Color::Red, Color::Green, Color::Blue]
        .repeat(1000)
        .to_vec();
    let s = TestStruct {
        value: 123,
        color: [
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Blue,
        ]
        .to_vec(),
        s_color: large_vec.as_slice(),
        metric: [Metric::Km, Metric::Liters, Metric::Temperature]
            .repeat(2000)
            .to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.color, ds.color);
    assert_eq!(s.s_color, ds.s_color);
    assert_eq!(s.metric, ds.metric);
}

#[test]
fn check_enum_vec_and_slice_u32align() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }

    #[flat_message(metadata: false, store_name: false)]
    struct TestStruct<'a> {
        value: u8,
        #[flat_message(repr = u8, kind = enum)]
        s_color: &'a [Color],
        #[flat_message(repr = u32, kind = enum)]
        metric: Vec<Metric>,
    }
    let mut v = Storage::default();
    let s = TestStruct {
        value: 123,
        s_color: &[Color::Red, Color::Green, Color::Blue],
        metric: [Metric::Km, Metric::Liters, Metric::Temperature].to_vec(),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = TestStruct::deserialize_from(&v).unwrap();
    assert_eq!(s.value, ds.value);
    assert_eq!(s.s_color, ds.s_color);
    assert_eq!(s.metric, ds.metric);
    assert_eq!(
        v.as_slice(),
        [
            71, 84, 72, 1, 3, 0, 0, 0, 213, 43, 122, 128, 3, 0, 0, 0, 1, 0, 0, 0, 128, 150, 152, 0,
            0, 202, 154, 59, 123, 237, 103, 151, 167, 3, 1, 10, 100, 0, 0, 0, 1, 211, 94, 66, 149,
            67, 175, 201, 147, 206, 4, 209, 28, 8, 29
        ]
    );
}


#[test]
fn check_serde_buffer_bool() {
    #[flat_message(metadata: false)]
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
fn check_serde_vec_str() {
    #[flat_message(metadata: false, store_name: false)]
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

    assert_eq!(v.as_slice(), &[
        /* Header                      */ 71, 84, 72, 1, 2, 0, 0, 0, 
        /* TestStruct: value           */ 64, 226, 1, 0, 
        /* v1                          */
        /* v1 (items count)            */ 4, 
        /* v1.item[0].len              */ 5, 
        /* v1.item[0].data             */ 72, 101, 108, 108, 111, // Hello
        /* v1.item[1].len              */ 5,
        /* v1.item[1].data             */ 87, 111, 114, 108, 100, // World
        /* v1.item[2].len              */ 4,
        /* v1.item[2].data             */ 74, 111, 104, 110, // John
        /* v1.item[3].len              */ 3,
        /* v1.item[3].data             */ 68, 111, 101, // Doe
        /* alignamnt                   */ 0, 0,
        /* Hash for TestStruct::value  */ 3, 211, 94, 66, 
        /* Hash for TestStruct::v1     */ 142, 70, 74, 148, 
        /* Offset of TestStruct::value */ 8,
        /* Offset of TestStruct::v1    */ 12
    ]);
}