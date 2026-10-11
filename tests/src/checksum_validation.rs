use crate::*;
use flat_message::*;

#[test]
fn simple_check() {
    #[derive(FlatMessage, Debug, PartialEq)]
    #[flat_message_options(checksum = true)]
    struct ImportantData {
        value: u64,
        message: String,
    }

    let data = ImportantData {
        value: 12345,
        message: "Critical information".to_string(),
    };

    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap();
    let restored = ImportantData::deserialize_from(&storage).unwrap();
    assert_eq!(data, restored);
}

#[test]
fn check_corruption_detection() {
    #[derive(FlatMessage, Debug, PartialEq)]
    #[flat_message_options(checksum = true)]
    struct ImportantData {
        value: u64,
        message: String,
    }
    let original = ImportantData {
        value: 42,
        message: "Hello World".to_string(),
    };

    let mut storage = Storage::default();
    original
        .serialize_to(&mut storage, Config::default())
        .unwrap();

    // Simulate corruption by modifying bytes
    let mut corrupted_bytes = storage.as_slice().to_vec();
    corrupted_bytes[10] = 0xFF; // Corrupt a byte
    let corrupted_storage = Storage::from_buffer(&corrupted_bytes);

    // Attempt to deserialize corrupted data
    match ImportantData::deserialize_from(&corrupted_storage) {
        Err(Error::InvalidChecksum((actual, expected))) => {
            println!("Corruption detected!");
            println!("Expected checksum: 0x{:08X}", expected);
            println!("Actual checksum: 0x{:08X}", actual);
        }
        Ok(_) => panic!("Should have detected corruption"),
        Err(e) => panic!("Other error: {}", e),
    }
}

#[test]
fn check_serialization_checksum() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false)]
    struct TestStruct1 {
        value: u32,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct TestStruct2 {
        value: u32,
    }
    let v1 = TestStruct1 { value: 123456 };
    let v2 = TestStruct2 { value: 123456 };
    let mut storage = Storage::default();
    v1.serialize_to(&mut storage, Config::default()).unwrap();
    let expected_output = vec![
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 140, 119, 161, 165,
    ];
    assert_eq!(storage.as_slice(), expected_output);
    let len_v1 = storage.len();
    v2.serialize_to(&mut storage, Config::default()).unwrap();
    let expected_output = vec![70, 76, 77, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8];
    assert_eq!(storage.as_slice(), expected_output);
    let len_v2 = storage.len();
    // TestStruct1 has 4 bytes more than TestStruct2 (for the checksum)
    assert_eq!(len_v1, len_v2 + 4);
}

#[test]
fn check_serde_with_checksum() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false)]
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
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false, validate_checksum = always)]
    struct TestStruct {
        value: u32,
    }
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 140, 119, 161, 165,
    ]);
    // all should be ok (as the checksum is correct)
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // this buffer has an invalid checksum (255, 255, 255, 255)
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
    ]);
    let v = TestStruct::deserialize_from(&buffer);
    match v.err() {
        Some(flat_message::Error::InvalidChecksum(_)) => {}
        _ => panic!("Invalid error - expected InvalidChecksum"),
    }
}

#[test]
fn check_deserialization_checksum_auto() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 140, 119, 161, 165,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // invalid checksum
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 255, 255, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer);
    match v.err() {
        Some(flat_message::Error::InvalidChecksum(_)) => {}
        _ => panic!("Invalid error - expected InvalidChecksum"),
    }
    // checksum is missing
    let buffer =
        Storage::from_buffer(&[70, 76, 77, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
}

#[test]
fn check_deserialization_checksum_ignore() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false, validate_checksum = ignore)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // invalid checksum (deserialization should still happen)
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
    ]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
    // checksum is missing
    let buffer =
        Storage::from_buffer(&[70, 76, 77, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = TestStruct::deserialize_from(&buffer).unwrap();
    assert_eq!(v.value, 123456);
}

#[test]
fn check_deserialization_checksum_unchecked_always() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(checksum = true, store_name = false, validate_checksum = always)]
    struct TestStruct {
        value: u32,
    }
    // valid checksum
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
    ]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
    // invalid checksum (deserialization should still happen)
    let buffer = Storage::from_buffer(&[
        70, 76, 77, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
    ]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
    // checksum is missing (deserialization should still happen)
    let buffer =
        Storage::from_buffer(&[70, 76, 77, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
    let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
    assert_eq!(v.value, 123456);
}
