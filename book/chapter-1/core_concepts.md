# Core Concepts

FlatMessage relies on a few core concepts / components that are used to build the serialization and deserialization logic.

## Storage

`Storage` is FlatMessage's primary buffer type for holding serialized data. It provides efficient memory management optimized for serialization workloads.

```rust
use flat_message::*;

// Using Storage for serialization
let mut storage = Storage::default();
data.serialize_to(&mut storage, Config::default())?;
```

**Storage advantages:**
- **Memory alignment**: Uses `Vec<u128>` internally for better alignment
- **Reduced allocations**: More efficient memory growth patterns
- **Optimized for serialization**: Designed specifically for FlatMessage workloads


### Storage API

```rust
impl Storage {
    // Create from existing byte buffer 
    pub fn from_buffer(input: &[u8]) -> Storage;
    
    // Create with a given capacity filled with zeros
    pub fn with_capacity(capacity: usize) -> Storage;

    // Get current size
    pub fn len(&self) -> usize;
}
```

Storage also implements `Default` trait, which creates an empty storage (with no allocated memory).



## Config

`Config` controls serialization behavior and constraints:

```rust
use flat_message::*;

// Default configuration
let config = Config::default();

// Custom configuration
let config = ConfigBuilder::new()
    .max_size(1024 * 1024)  // 1MB limit
    .build();
```

### Configuration Options

You can use `ConfigBuilder` to create a `Config` instance and provide a set of options (on how the serialization should be performed).

| Option     | Default | Description                                                                                                  |
| ---------- | ------- | ------------------------------------------------------------------------------------------------------------ |
| `max_size` | 16MB    | Maximum serialized size allowed (in bytes). If the serialized size exceeds this limit, an error is returned. |

### Using Config

```rust
#[derive(FlatMessage)]
struct Data {
    content: Vec<u8>,
}

let data = Data { content: vec![1, 2, 3] };
let mut storage = Storage::default();

// Use custom size limit
let config = ConfigBuilder::new()
    .max_size(1024)  // Only allow 1KB
    .build();

match data.serialize_to(&mut storage, config) {
    Ok(()) => println!("Serialization successful"),
    Err(Error::ExceedMaxSize((actual, max))) => {
        println!("Data too large: {} bytes (max: {})", actual, max);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## FlatMessage Trait

The `FlatMessage` trait defines the core serialization interface:

```rust
pub trait FlatMessage<'a> {
    // Serialize data to a Storage buffer
    fn serialize_to(&self, output: &mut Storage, config: Config) -> Result<(), Error>;
    
    // Deserialize data from a buffer (with validation)
    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
    
    // Deserialize without validation (faster, but unsafe)
    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
}
```

### Serialization

Serialization transforms your struct into a byte buffer:

```rust
#[derive(FlatMessage)]
struct Point {
    x: f32,
    y: f32,
}

let point = Point { x: 1.0, y: 2.0 };
let mut storage = Storage::default();

// Serialize the point
point.serialize_to(&mut storage, Config::default())?;

// Access the raw bytes
let bytes = storage.as_slice();
println!("Serialized size: {} bytes", bytes.len());
```

### Deserialization

Deserialization reconstructs your struct from bytes:

```rust
// Deserialize with validation (recommended)
let restored_point = Point::deserialize_from(&storage)?;

// Deserialize without validation (faster, but risky)
let restored_point = unsafe {
    Point::deserialize_from_unchecked(&storage)?
};
```

## Zero-Copy Deserialization

FlatMessage's key feature is **zero-copy deserialization** - it doesn't copy data from the buffer when possible. This is however highly dependent on the data type you are deserializing. Some of them such as `String` or `Vec<T>` require allocation and copying of the data. Also, basic types such as `u32`, `f32`, `bool` etc. are not zero-copy types (but since they are small, the performance impact is negligible).

```rust
#[derive(FlatMessage)]
struct Message<'a> {
    title: &'a str,        // Points directly into buffer
    tags: &'a [u32],       // Points directly into buffer
    owned_data: String,    // This still requires allocation
}
```

### Zero-Copy vs Allocation

| Type             | Zero-Copy | Notes                          |
| ---------------- | --------- | ------------------------------ |
| `&str`           | ✅ Yes     | Points into original buffer    |
| `&[T]`           | ✅ Yes     | Points into original buffer    |
| `&[u8; N]`       | ✅ Yes     | Points into original buffer    |
| `String`         | ❌ No      | Requires allocation and copy   |
| `Vec<T>`         | ❌ No      | Requires allocation and copy   |
| `SmallVec<[T; N]>` | 🟨 Partial | Requires copy, but no allocation unless >N items (*) |
| `Option<&str>`   | ✅ Yes     | When Some, points into buffer  |
| `Option<String>` | ❌ No      | When Some, requires allocation |

(*): Uses the [smallvec](https://docs.rs/smallvec/latest/smallvec/) crate, and requires the `smallvec` feature. Enables zero-allocation shallow copies, in which items can be zero-copy (e.g. in a `SmallVec<[&'a str, N]>`), useful when the vector itself cannot be.

### Lifetime Management

Zero-copy deserialization means your deserialized struct borrows from the original buffer:

```rust
// We assume that the `get_serialized_data` function returns a `Storage` object that contains the serialized data.
fn correct_process_message() -> Result<(), Error> {
    let storage = get_serialized_data();
    
    // This works - storage lives long enough
    let message = Message::deserialize_from(&storage)?;
    println!("Title: {}", message.title);
    
    Ok(())
} // storage and message both dropped here

fn broken_process_message() -> Result<Message<'static>, Error> {
    let storage = get_serialized_data();
    let message = Message::deserialize_from(&storage)?;
    
    // This won't compile - message can't outlive storage
    // Ok(message) // ❌ Compilation error
    
    unreachable!()
}
```

## Performance Characteristics

Understanding performance implications helps you make good design decisions:

### Serialization Performance

- **Direct types** (u32, f64, bool): Fastest, just memory copies
- **Strings**: Fast, just memory copies (for both `&str` and `String`)
- **Vectors/Slices**: Fast, just memory copies
- **Enums**: Fast, just the underlying representation

### Deserialization Performance

- **Zero-copy types**: Fastest, just pointer adjustments
- **Owned types**: Slower, requires allocation and copying
- **Validation**: `deserialize_from()` validates data, `deserialize_from_unchecked()` skips validation

### Memory Usage

Being a schemaless library, FlatMessage has to store information on the type of the data being serialized. This is done by storing a hash over then type name and the type size. 

As a general rule, for a structure with `n` fields, the serialized size will:
- `8 bytes` for the header
- `5 bytes` x `n` for the fields (for each field, 5 bytes are used to store the field name and the field size)
- `x bytes` the actual content of the fields (where `x` is the sum of the sizes of all the fields)

Additionally, the following information is stored (if enabled)
- `4 bytes` for the checksum (if enabled)
- `4 bytes` for the name of the structure (if enabled)
- `8 bytes` for an unique identifier (if enabled)
- `8 bytes` for the timestamp of the serialization (if enabled)

This means that for the following structure:

```rust

#[derive(FlatMessage)]
struct Point {
    x: u32,
    y: u32,
}
```

The serialized size will be:
- `8 bytes` for the header
- `5 bytes` x `2` for the fields (for each field, 5 bytes are used to store the field name and the field size)
- `8 bytes` for the actual content of the fields (both fields are `u32`, so 8 bytes each)
- `4 bytes` for the name of the structure (enabled by default)

So a total of `30 bytes` for the serialized size. The size could be **4 bytes smaller** if we add `#[flat_message_options(store_name = false)]` to the structure.

**Remarks:**
- The increase of size is not linear, but rather it depends on the number of fields and their sizes. For example a structure with only one string field that holds 1000 characters will only add 13 ore characters on the serialized buffer (1013 bytes) witch is insignificant relative to the actual content of the field.



## Best Practices

1. **Use Storage for serialization**: Storage is the only supported serialization target, optimized for FlatMessage workloads
2. **Prefer zero-copy types**: Use `&str` over `String`, `&[T]` over `Vec<T>` when possible. For lists of zero-copy types, consider using the [smallvec](https://docs.rs/smallvec/latest/smallvec/) crate with the `smallvec` feature.
3. **Validate when needed**: Use `deserialize_from()` for untrusted data, `deserialize_from_unchecked()` for performance-critical trusted data
4. **Set appropriate limits**: Use Config to prevent excessive memory usage
5. **Manage lifetimes carefully**: Ensure buffers live long enough for zero-copy data

## Example: Complete Workflow

```rust
use flat_message::*;

#[derive(FlatMessage, Debug, PartialEq)]
struct NetworkMessage<'a> {
    session_id: u64,
    command: &'a str,      // Zero-copy
    payload: &'a [u8],     // Zero-copy  
}

fn network_example() -> Result<(), flat_message::Error> {
    // Create message
    let original_message = NetworkMessage {
        session_id: 12345,
        command: "GET_USER",
        payload: &[1, 2, 3, 4, 5],
    };

    // Serialize with size limit
    let mut storage = Storage::default();
    let config = ConfigBuilder::new()
        .max_size(1024)  // 1KB limit
        .build();
    
    original_message.serialize_to(&mut storage, config)?;
    
    // Send over network (simulation)
    let network_bytes = storage.as_slice().to_vec();
    
    // Receive and deserialize
    let received_storage = Storage::from_buffer(&network_bytes);
    let received_message = NetworkMessage::deserialize_from(&received_storage)?;
    
    assert_eq!(original_message, received_message);
    println!("Message transmitted successfully!");
    
    Ok(())
}
```