use super::*;
use flat_message::*;
use std::fmt::Debug;

// macro_rules! check_field_value {
//     ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
//         let val: $type = $flat_message_buffer.get($field_name).unwrap();
//         assert_eq!(val, $value);
//     };
// }
// macro_rules! check_field_value_unsafe {
//     ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
//         let val: $type = unsafe { $flat_message_buffer.get_unchecked($field_name).unwrap() };
//         assert_eq!(val, $value);
//     };
// }

// #[test]
// fn check_flat_message_buffer_one_field_i32() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         my_field: i32,
//     }
//     let a = TestStruct {
//         my_field: 123456,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("my_field"), i32, 123456, buf);
// }

// #[test]
// fn check_flat_message_buffer_one_field_str() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         my_field: String,
//     }
//     let a = TestStruct {
//         my_field: "Hello, World!".to_string(),
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("my_field"), &str, "Hello, World!", buf);
// }

// #[test]
// fn check_flat_message_buffer_two_fields_i32_i8() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         size: i32,
//         dimension: i8,
//     }
//     let a = TestStruct {
//         size: -12345,
//         dimension: -100,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("size"), i32, -12345, buf);
//     check_field_value!(name!("dimension"), i8, -100, buf);
// }

// #[test]
// fn check_flat_message_buffer_two_fields_string_string() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("name"), &str, "John", buf);
//     check_field_value!(name!("surname"), &str, "Doe", buf);
// }

// #[test]
// fn check_flat_message_buffer_safe() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("name"), &str, "John", buf);
//     check_field_value!(name!("surname"), &str, "Doe", buf);
//     check_field_value!(name!("math"), u8, 100, buf);
//     check_field_value!(name!("engligh"), u8, 90, buf);
//     check_field_value!(name!("passed"), bool, true, buf);
//     check_field_value!(name!("average"), f64, 95.0, buf);
// }

// #[test]
// fn check_flat_message_buffer_unsafe() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value_unsafe!(name!("name"), &str, "John", buf);
//     check_field_value_unsafe!(name!("surname"), &str, "Doe", buf);
//     check_field_value_unsafe!(name!("math"), u8, 100, buf);
//     check_field_value_unsafe!(name!("engligh"), u8, 90, buf);
//     check_field_value_unsafe!(name!("passed"), bool, true, buf);
//     check_field_value_unsafe!(name!("average"), f64, 95.0, buf);
// }

// // #[test]
// // fn check_serde_full() {
// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u8,
// //         passed: bool,
// //         average: f64,
// //     }
// //     let mut a = TestStruct {
// //         name: "John".to_string(),
// //         surname: "Doe",
// //         math: 100,
// //         engligh: 90,
// //         passed: true,
// //         average: 95.0,
// //     };
// //     a.update_metada(MetaDataBuilder::new()
// //         .timestamp(123456)
// //         .unique_id(654321)
// //         .build());
// //     let mut output = Storage::default();
// //     a.serialize_to(&mut output, Config::default()).unwrap();
// //     let b = TestStruct::deserialize_from(&output).unwrap();
// //     assert_eq!(a.name, b.name);
// //     assert_eq!(a.surname, b.surname);
// //     assert_eq!(a.math, b.math);
// //     assert_eq!(a.engligh, b.engligh);
// //     assert_eq!(a.passed, b.passed);
// //     assert_eq!(a.average, b.average);
// //     assert_eq!(a.metadata().timestamp(), b.metadata().timestamp());
// //     assert_eq!(a.metadata().unique_id(), b.metadata().unique_id());
// // }

// #[test]
// fn check_serde_full_unchecked() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let mut a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     a.update_metada(MetaDataBuilder::new()
//         .timestamp(123456)
//         .unique_id(654321)
//         .build());
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let b = unsafe { TestStruct::deserialize_from_unchecked(&output).unwrap() };
//     assert_eq!(a.name, b.name);
//     assert_eq!(a.surname, b.surname);
//     assert_eq!(a.math, b.math);
//     assert_eq!(a.engligh, b.engligh);
//     assert_eq!(a.passed, b.passed);
//     assert_eq!(a.average, b.average);
//     assert_eq!(a.metadata().timestamp(), b.metadata().timestamp());
//     assert_eq!(a.metadata().unique_id(), b.metadata().unique_id());
// }

// #[test]
// fn check_structure_information() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(version = 12)]
//     struct TestStruct {
//         a: u64,
//         b: u32,
//     }
//     let mut a = TestStruct {
//         a: 12,
//         b: 34,
//     };
//     a.update_metada(MetaDataBuilder::new()
//         .timestamp(123456)
//         .unique_id(654321)
//         .build());
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let si = StructureInformation::try_from(&output).unwrap();
//     assert_eq!(si.timestamp(), Some(123456));
//     assert_eq!(si.unique_id(), Some(654321));
//     assert_eq!(si.version(), Some(12));
//     assert_eq!(si.name(), Some(name!("TestStruct")));
// }

// #[test]
// fn check_clone() {
//     #[derive(Clone, Debug, Eq, PartialEq, FlatMessage)]
//     struct TestStruct {
//         a: String,
//         b: String,
//     }
//     let mut v1 = TestStruct {
//         a: "Hello".to_string(),
//         b: "World".to_string(),
//     };
//     v1.update_metada(MetaDataBuilder::new().timestamp(1).unique_id(2).build());
//     let v2 = v1.clone();
//     assert_eq!(v1.a, v2.a);
//     assert_eq!(v1.b, v2.b);
//     assert_eq!(v1.metadata(), v2.metadata());
//     assert_eq!(v1, v2);
//     let mut storage = Storage::default();
//     v1.serialize_to(&mut storage, Config::default()).unwrap();
//     let v3 = TestStruct::deserialize_from(&storage).unwrap();
//     assert_eq!(v1, v3);
// }

#[test]
fn check_max_size_for_serialization() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct TestStruct {
        value: u32,
    }
    let mut v = Storage::default();
    let s = TestStruct { value: 123456 };
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
fn check_simple_struct_width_comments() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Point {
        // x coordinate
        x: i32,
        // y coordinate
        y: i32,
    }
    validate_correct_serde(Point { x: 10, y: 20 });
}

#[test]
fn check_simple_struct_width_documentation() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Point {
        /// x coordinate that is used to store the position
        /// in the 2D space
        x: i32,
        /// y coordinate that is used to store the position
        /// in the 2D space
        y: i32,
    }
    validate_correct_serde(Point { x: 10, y: 20 });
}

#[test]
fn check_task_example() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
    }

    #[derive(FlatMessage, Debug, PartialEq)]
    #[flat_message_options(version = 1, store_name = true, checksum = true)]
    struct Task {
        title: String,
        description: Option<String>,
        completed: bool,

        #[flat_message_item(repr = u8, kind = enum)]
        priority: Priority,

        tags: Vec<String>,
    }

    let task = Task {
        title: "Learn FlatMessage".to_string(),
        description: Some("Read the documentation".to_string()),
        completed: false,
        priority: Priority::High,
        tags: vec!["learning".to_string(), "rust".to_string()],
    };

    // Create a serialization storage buffer
    let mut storage = Storage::default();
    if let Err(e) = task.serialize_to(&mut storage, Config::default()) {
        panic!("Error serializing task: {}", e);
    }

    // print the buffer
    println!("Buffer: {:?}", storage.as_slice());

    // Deserialize from buffer
    match Task::deserialize_from(&storage) {
        Ok(restored_task) => {
            assert_eq!(task, restored_task);
            println!("Task serialized and deserialized successfully");
        }
        Err(e) => {
            panic!("Error deserializing task: {}", e);
        }
    }
}

#[test]
fn check_config_max_size() {
    #[derive(FlatMessage)]
    #[flat_message_options(optimized_unchecked_code = false)]
    struct Data {
        content: Vec<u8>,
    }

    let data = Data {
        content: vec![1, 2, 3],
    };
    let mut storage = Storage::default();
    let config = ConfigBuilder::new().max_size(10).build();
    let result = data.serialize_to(&mut storage, config);
    assert_eq!(result, Err(flat_message::Error::ExceedMaxSize((21, 10))));
}

#[test]
fn check_config_skip_field() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Data {
        x: u8,
        #[flat_message_item(ignore = true)]
        y: u32,
    }

    let data = Data { x: 1, y: 2 };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap();
    let ds = Data::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.y, 0); // not 2 -> 0 is the default for y
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
fn check_config_skip_unknown_field() {
    #[derive(Default)]
    struct MyData {
        a: u8,
        b: u32,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Data {
        x: u8,
        #[flat_message_item(ignore = true)]
        y: MyData,
    }

    let data = Data {
        x: 1,
        y: MyData { a: 2, b: 3 },
    };
    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default()).unwrap();
    let ds = Data::deserialize_from(&storage).unwrap();
    assert_eq!(ds.x, 1);
    assert_eq!(ds.y.a, 0);
    assert_eq!(ds.y.b, 0);
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
fn check_mandatory_field() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        #[flat_message_item(mandatory = false)]
        c: u16,
        d: String,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2 {
        a: u8,
        b: u32,
        // c was deleted
        d: String,
    }
    let mut storage = Storage::default();
    let data_v2 = MyDataV2 {
        a: 1,
        b: 2,
        d: "Hello".to_string(),
    };
    data_v2
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let data_v1 = MyDataV1::deserialize_from(&storage).unwrap();
    assert_eq!(data_v1.a, 1);
    assert_eq!(data_v1.b, 2);
    assert_eq!(data_v1.c, 0); // c is not mandatory, so it is defaulted to 0
    assert_eq!(data_v1.d, "Hello".to_string());
}

#[test]
fn check_without_mandatory_field() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2 {
        a: u8,
        b: u32,
        // c was deleted
        d: String,
    }
    let mut storage = Storage::default();
    let data_v2 = MyDataV2 {
        a: 1,
        b: 2,
        d: "Hello".to_string(),
    };
    data_v2
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let result = MyDataV1::deserialize_from(&storage);
    assert!(result.is_err());
    match result.err() {
        Some(flat_message::Error::FieldIsMissing(_)) => {}
        _ => panic!("Invalid error - expected FieldIsMissing"),
    }
}

#[test]
fn check_mandatory_string_reference_field() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = "false")]
    struct MyDataV1 {
        a: u8,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2<'a> {
        a: u8,
        #[flat_message_item(mandatory = false)]
        b: &'a str,
    }
    let mut storage = Storage::default();
    let data_v1 = MyDataV1 { a: 1 };
    data_v1
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let data_v2 = MyDataV2::deserialize_from(&storage).unwrap();
    assert_eq!(data_v2.a, 1);
    assert_eq!(data_v2.b, "");
}

#[test]
fn check_mandatory_default_value_for_string_reference_field() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = "false")]
    struct MyDataV1 {
        a: u8,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2<'a> {
        a: u8,
        #[flat_message_item(mandatory = false, default = "Hello")]
        b: &'a str,
    }
    let mut storage = Storage::default();
    let data_v1 = MyDataV1 { a: 1 };
    data_v1
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let data_v2 = MyDataV2::deserialize_from(&storage).unwrap();
    assert_eq!(data_v2.a, 1);
    assert_eq!(data_v2.b, "Hello");
}

#[test]
fn check_mandatory_default_value_for_a_vector() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = "false")]
    struct MyDataV1 {
        a: u8,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2 {
        a: u8,
        #[flat_message_item(mandatory = false, default = "vec![1,2,3]")]
        b: Vec<u8>,
    }
    let mut storage = Storage::default();
    let data_v1 = MyDataV1 { a: 1 };
    data_v1
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let data_v2 = MyDataV2::deserialize_from(&storage).unwrap();
    assert_eq!(data_v2.a, 1);
    assert_eq!(data_v2.b, vec![1, 2, 3]);
}

#[test]
fn check_mandatory_default_value_for_a_u32() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = "false")]
    struct MyDataV1 {
        a: u8,
    }
    const DEFAULT_VALUE: u32 = 1234567890;
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct MyDataV2 {
        a: u8,
        #[flat_message_item(mandatory = false, default = "DEFAULT_VALUE")]
        b: u32,
    }
    let mut storage = Storage::default();
    let data_v1 = MyDataV1 { a: 1 };
    data_v1
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let data_v2 = MyDataV2::deserialize_from(&storage).unwrap();
    assert_eq!(data_v2.a, 1);
    assert_eq!(data_v2.b, DEFAULT_VALUE);
}

#[test]
fn check_vec_to_slice_interchangeability() {
    use flat_message::*;

    #[derive(FlatMessage)]
    struct DataWriter {
        numbers: Vec<u32>,
        names: Vec<String>,
    }
    #[derive(FlatMessage)]
    struct DataReader<'a> {
        numbers: &'a [u32],
        names: Vec<&'a str>,
    }
    let writer_data = DataWriter {
        numbers: vec![1, 2, 3, 4, 5],
        names: vec!["Alice".to_string(), "Bob".to_string()],
    };
    let mut storage = Storage::default();
    writer_data
        .serialize_to(&mut storage, Config::default())
        .unwrap();
    let reader_data = DataReader::deserialize_from(&storage).unwrap();

    assert_eq!(reader_data.numbers, &[1, 2, 3, 4, 5]);
    assert_eq!(reader_data.names, &["Alice", "Bob"]);
}

#[test]
fn check_option_to_non_option_interchangeability() {
    #[derive(FlatMessage)]
    struct OptionalData {
        required_field: u32,
        optional_field: Option<String>,
        optional_number: Option<i64>,
    }

    #[derive(FlatMessage)]
    struct RequiredData {
        required_field: u32,
        optional_field: String, // Must exist in data
        optional_number: i64,   // Must exist in data
    }

    let opt_data = OptionalData {
        required_field: 42,
        optional_field: Some("Hello".to_string()),
        optional_number: Some(123),
    };

    let mut storage = Storage::default();
    opt_data
        .serialize_to(&mut storage, Config::default())
        .unwrap();

    let req_data = RequiredData::deserialize_from(&storage).unwrap();
    assert_eq!(req_data.optional_field, "Hello");
    assert_eq!(req_data.optional_number, 123);
}

#[test]
fn check_string_to_non_string_interchangeability() {
    #[derive(FlatMessage)]
    struct StringWriter {
        owned_text: String,
        optional_text: Option<String>,
    }

    #[derive(FlatMessage)]
    struct StringReader<'a> {
        owned_text: &'a str,    // String -> &str (zero-copy)
        optional_text: &'a str, // Option<String> -> &str (if Some)
    }

    let writer = StringWriter {
        owned_text: "Hello World".to_string(),
        optional_text: Some("Optional text".to_string()),
    };

    let mut storage = Storage::default();
    writer
        .serialize_to(&mut storage, Config::default())
        .unwrap();

    let reader = StringReader::deserialize_from(&storage).unwrap();
    assert_eq!(reader.owned_text, "Hello World");
    assert_eq!(reader.optional_text, "Optional text");
}

#[test]
fn check_serde_into_smaller_struct() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[derive(Debug, PartialEq, FlatMessage)]
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
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[derive(Debug, PartialEq, FlatMessage)]
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
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestSmallerStruct::deserialize_from(&output);
    assert!(b.is_err());
}

#[test]
fn check_serde_into_different_type() {
    #[derive(Debug, PartialEq, FlatMessage)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }

    #[derive(Debug, PartialEq, FlatMessage)]
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
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let b = TestStruct2::deserialize_from(&output);
    assert!(b.is_err());
}

#[test]
fn check_readme_example_1() {
    #[derive(FlatMessage, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    let mut storage = Storage::default();
    person.serialize_to(&mut storage, Config::default()).unwrap();

    // Deserialize
    let restored = Person::deserialize_from(&storage).unwrap();
    assert_eq!(person, restored);

    println!("Serialized {} bytes", storage.len());
}

#[test]
fn check_readme_example_2() {
    #[derive(FlatMessage)]
    struct Message<'a> {
        title: &'a str,        // Zero-copy string reference
        tags: &'a [u32],       // Zero-copy slice reference
        metadata: &'a [u8],    // Zero-copy byte slice
    }
    #[derive(FlatMessage)]
    struct MessageOwned {
        title: String,
        tags: Vec<u32>,
        metadata: Vec<u8>,
    }
    let owned_data = MessageOwned {
        title: "Hello World".to_string(),
        tags: vec![1, 2, 3, 4, 5],
        metadata: vec![0xFF, 0xFE, 0xFD],
    };

    let mut storage = Storage::default();
    owned_data.serialize_to(&mut storage, Config::default()).unwrap();

    // Deserialize with zero-copy references
    let message = Message::deserialize_from(&storage).unwrap();
    
    // No data copying - direct buffer access!
    println!("Title: {}", message.title);      // Points into storage
    println!("Tags: {:?}", message.tags);      // Points into storage
    println!("Metadata: {:?}", message.metadata); // Points into storage

    assert_eq!(message.title, "Hello World");
    assert_eq!(message.tags, &[1, 2, 3, 4, 5]);
    assert_eq!(message.metadata, &[0xFF, 0xFE, 0xFD]);
}