# Type Interchangeability

One of FlatMessage's powerful features is the ability to use different but compatible types during serialization and deserialization. This flexibility enables efficient memory usage and performance optimization.

## 1. Vec vs Slice Interchangeability

You can serialize data with `Vec<T>` and deserialize it as `&[T]`, or vice versa:

```rust
use flat_message::*;

// Serialization struct using Vec
#[derive(FlatMessage)]
struct DataWriter {
    numbers: Vec<u32>,
    names: Vec<String>,
}

// Deserialization struct using slices (zero-copy)
#[derive(FlatMessage)]
struct DataReader<'a> {
    numbers: &'a [u32],      // Zero-copy reference
    names: Vec<&'a str>,,    // Zero-copy reference for &str, allocation for Vec
}

fn example() -> Result<(), Error> {
    // Create data with Vec (ownership)
    let writer_data = DataWriter {
        numbers: vec![1, 2, 3, 4, 5],
        names: vec!["Alice".to_string(), "Bob".to_string()],
    };

    // Serialize
    let mut storage = Storage::default();
    writer_data.serialize_to(&mut storage, Config::default())?;

    // Deserialize as slices (zero-copy)
    let reader_data = DataReader::deserialize_from(&storage)?;
    
    println!("Numbers: {:?}", reader_data.numbers);  // [1, 2, 3, 4, 5]
    println!("Names: {:?}", reader_data.names);      // ["Alice", "Bob"]
    
    Ok(())
}
```

### Performance Implications

| Type               | Serialization        | Deserialization                      | Memory Usage        |
| ------------------ | -------------------- | ------------------------------------ | ------------------- |
| `Vec<T>`           | Moderate (iteration) | Slow (allocation + copy)             | High (owned data)   |
| `SmallVec<[T; N]>` | Moderate (iteration) | Moderate (copy, possibly allocation) | High (owned data)   |
| `&[T]`             | Fast (direct copy)   | Fast (zero-copy)                     | Low (borrowed data) |

**Best Practice**: Use `Vec<T>` for data you own/modify, `&[T]` for read-only access where zero-copy deserialization is possible, and consider using `SmallVec<[T; N]>` ([smallvec](https://docs.rs/smallvec/latest/smallvec/) crate with the `smallvec` feature) when the number of items is usually small, especially if they contain borrowed data.

## 2. Option vs Non-Option Interchangeability

Fields can be compatible between `Option<T>` and `T` under certain conditions:

```rust
// Serialization with Option
#[derive(FlatMessage)]
struct OptionalData {
    required_field: u32,
    optional_field: Option<String>,
    optional_number: Option<i64>,
}

// Deserialization without Option
#[derive(FlatMessage)]
struct RequiredData {
    required_field: u32,
    optional_field: String,  // Must exist in data
    optional_number: i64,    // Must exist in data
}

fn option_compatibility() -> Result<(), Error> {
    // Serialize with Some values
    let opt_data = OptionalData {
        required_field: 42,
        optional_field: Some("Hello".to_string()),
        optional_number: Some(123),
    };

    let mut storage = Storage::default();
    opt_data.serialize_to(&mut storage, Config::default())?;

    // Deserialize as required fields
    let req_data = RequiredData::deserialize_from(&storage)?;
    assert_eq!(req_data.optional_field, "Hello");
    assert_eq!(req_data.optional_number, 123);

    Ok(())
}
```

### Compatibility Rules

| Serialized As | Deserialized As | Result                      |
| ------------- | --------------- | --------------------------- |
| `Some(value)` | `T`             | ✅ Works, gets `value`       |
| `None`        | `T`             | ❌ Error: field missing      |
| `T`           | `Option<T>`     | ✅ Works, gets `Some(value)` |

### Handling None Values

When deserializing `None` as a required field, you get an error:

```rust
fn handle_missing_optional() -> Result<(), Error> {
    let opt_data = OptionalData {
        required_field: 42,
        optional_field: None,       // This will cause an error
        optional_number: Some(123),
    };

    let mut storage = Storage::default();
    opt_data.serialize_to(&mut storage, Config::default())?;

    // This will fail because optional_field is None
    match RequiredData::deserialize_from(&storage) {
        Err(Error::FieldIsMissing(_)) => {
            println!("Field was None, can't deserialize as required");
        }
        _ => {}
    }

    Ok(())
}
```

## 3. String vs &str Interchangeability

Similar rules apply to strings:

```rust
#[derive(FlatMessage)]
struct StringWriter {
    owned_text: String,
    optional_text: Option<String>,
}

#[derive(FlatMessage)]
struct StringReader<'a> {
    owned_text: &'a str,        // String -> &str (zero-copy)
    optional_text: &'a str,     // Option<String> -> &str (if Some)
}

fn string_example() -> Result<(), Error> {
    let writer = StringWriter {
        owned_text: "Hello World".to_string(),
        optional_text: Some("Optional text".to_string()),
    };

    let mut storage = Storage::default();
    writer.serialize_to(&mut storage, Config::default())?;

    let reader = StringReader::deserialize_from(&storage)?;
    println!("Text: {}", reader.owned_text);        // "Hello World"
    println!("Optional: {}", reader.optional_text); // "Optional text"

    Ok(())
}
```



## Compatibility Matrix

| Serialize           | Deserialize        | Compatible  | Performance | Notes                    |
| ---------           | -----------        | ----------- | ----------- | --------------------     |
| `Vec<T>`            | `Vec<T>`           | ✅          | Slow        | Copy required            |
| `Vec<T>`            | `&[T]`             | ✅          | Fast        | Zero-copy                |
| `Vec<T>`            | `SmallVec<[T; N]>` | ✅          | Average     | Zero-allocation possible |
| `&[T]`              | `Vec<T>`           | ✅          | Slow        | Copy required            |
| `&[T]`              | `&[T]`             | ✅          | Fast        | Zero-copy                |
| `&[T]`              | `SmallVec<[T; N]>` | ✅          | Average     | Zero-allocation possible |
| `SmallVec<[T; N1]>` | `SmallVec<[T; N2]>`| ✅          | Average     | Zero-allocation possible |
| `SmallVec<[T; N]>`  | `Vec<T>`           | ✅          | Slow        | Copy required            |
| `SmallVec<[T; N]>`  | `&[T]`             | ✅          | Fast        | Zero-copy                |
| `Some(T)`           | `T`                | ✅          | Same as T   | Direct access            |
| `None`              | `T`                | ❌          | -           | Error: field missing     |
| `T`                 | `Option<T>`        | ✅          | Same as T   | Wrapped in Some          |
| `String`            | `&str`             | ✅          | Fast        | Zero-copy                |
| `&str`              | `String`           | ✅          | Slow        | Copy required            |
