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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let buf = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = TestStruct::deserialize_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = TestSmallerStruct::deserialize_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = TestSmallerStruct::deserialize_from(output.as_slice());
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = TestStruct2::deserialize_from(output.as_slice());
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = TestStruct2::deserialize_from(output.as_slice()).unwrap();
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = unsafe { TestStruct::deserialize_from_unchecked(output.as_slice()).unwrap() };
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
    let mut output = Vec::new();
    a.serialize_to(&mut output);
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

    let mut output = Vec::new();
    a.serialize_to(&mut output);
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

    let mut output_1 = Vec::new();
    let mut output_2 = Vec::new();
    a_1.serialize_to(&mut output_1);
    a_2.serialize_to(&mut output_2);

    // from TestStruct1 to TestStruct1
    let b = TestStruct1::deserialize_from(output_1.as_slice()).unwrap();
    assert_eq!(a_1.value, b.value);

    // from TestStruct1 to TestStruct2 (no validation name required -> should be possible)
    let b = TestStruct2::deserialize_from(output_1.as_slice()).unwrap();
    assert_eq!(a_1.value, b.value);

    // from TestStruct2 to TestStruct1 (validation name required -> should not be possible)
    let b = TestStruct1::deserialize_from(output_2.as_slice());
    assert_eq!(b.is_err(), true);

    // from TestStruct2 to TestStruct2
    let b = TestStruct2::deserialize_from(output_2.as_slice()).unwrap();
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
    let mut o1 = Vec::new();
    let mut o2 = Vec::new();
    let mut o3 = Vec::new();
    v3::TestStruct {
        value: 3,
        metadata: MetaDataBuilder::new().timestamp(333).unique_id(33).build(),
    }
    .serialize_to(&mut o3);
    v2::TestStruct {
        value: 2,
        metadata: MetaDataBuilder::new().timestamp(222).unique_id(22).build(),
    }
    .serialize_to(&mut o2);
    v1::TestStruct {
        value: 1,
        metadata: MetaDataBuilder::new().timestamp(111).unique_id(11).build(),
    }
    .serialize_to(&mut o1);
    let v1_from_v3 = v1::TestStruct::deserialize_from(o3.as_slice());
    let v1_from_v2 = v1::TestStruct::deserialize_from(o2.as_slice());
    let v2_from_v3 = v2::TestStruct::deserialize_from(o3.as_slice());
    let v3_from_v1 = v3::TestStruct::deserialize_from(o1.as_slice());
    let v3_from_v2 = v3::TestStruct::deserialize_from(o2.as_slice());
    let v2_from_v1 = v2::TestStruct::deserialize_from(o1.as_slice());
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
    let mut storage = Vec::new();
    v1.serialize_to(&mut storage);
    let v3 = TestStruct::deserialize_from(storage.as_slice()).unwrap();
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
    let mut storage = Vec::new();
    v1.serialize_to(&mut storage);
    let v3 = TestStruct::deserialize_from(storage.as_slice()).unwrap();
    assert_eq!(v1, v3);
}