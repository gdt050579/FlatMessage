use flat_message::*;

macro_rules! check_field_value {
    ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
        let val: $type = $flat_message_buffer.get($field_name).unwrap();
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
fn check_flat_message_buffer_1() {
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
fn check_flat_message_metadata() {
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
