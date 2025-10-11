# FlatMessage

<img src="book/chapter-1/img/logo.png" width="1000" />

![Build Status](https://github.com/gdt050579/FlatMessage/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/flat_message.svg)](https://crates.io/crates/flat_message)
[![Documentation](https://docs.rs/flat_message/badge.svg)](https://docs.rs/flat_message)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

**FlatMessage** is a **zero-copy**, **schema-less** serialization library built for Rust, offering efficient and flexible data serialization with exceptional performance and rich type support.

## Key Features

- **Extremely Fast**: Up to **6.7 GB/sec** throughput (serialization and deserialization) - fastest schema-less serialization
- **Zero-Copy Deserialization**: Direct buffer access for `&str`, `&[T]`, and `&[u8; N]` types
- **Schema-Less**: No schema definition required - all type information embedded in data
- **Type Interchangeability**: Serialize as `Vec<T>`, deserialize as `&[T]` (and vice versa)
- **Rich Type System**: Comprehensive support for basic types, enums, variants, structs, and more
- **Version Compatibility**: Forward/backward compatibility with flexible version ranges
- **Production Ready**: Checksum validation, size limits, comprehensive error handling

## Performance Comparison

FlatMessage consistently outperforms other serialization libraries across different data structures:

| Library           | Type         | Throughput (MB/sec) | Notes                           |
| ----------------- | ------------ | ------------------- | ------------------------------- |
| **FlatMessage** ⚡ | Schema-less  | **4,624 - 6,705**   | (⚡) = Unchecked deserialization |
| **FlatMessage**   | Schema-less  | **3,889 - 5,073**   | With validation                 |
| protobuf          | Schema-based | 2,261 - 2,799       |                                 |
| postcard          | Schema-based | 2,213 - 2,960       |                                 |
| bincode           | Schema-based | 2,025 - 2,323       |                                 |
| flexbuffers       | Schema-less  | 410 - 582           |                                 |
| JSON              | Schema-less  | 342 - 479           |                                 |

*Averaged across Windows, macOS, and Linux on multiple test structures*
More details in the [performance benchmarks](https://gdt050579.github.io/FlatMessage/chapter-5/performance_results.html).

## Quick Start

Add FlatMessage to your `Cargo.toml`:

```toml
[dependencies]
flat_message = "*"
```

### Basic Usage

```rust
use flat_message::*;

#[derive(FlatMessage, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() -> Result<(), Error> {
    // Create and serialize
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    let mut storage = Storage::default();
    person.serialize_to(&mut storage, Config::default())?;

    // Deserialize
    let restored = Person::deserialize_from(&storage)?;
    assert_eq!(person, restored);

    println!("Serialized {} bytes", storage.len());
    Ok(())
}
```

### Zero-Copy Deserialization

```rust
use flat_message::*;

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

fn zero_copy_example() -> Result<(), Error> {
    // Serialize with owned data
    let owned_data = MessageOwned {
        title: "Hello World".to_string(),
        tags: vec![1, 2, 3, 4, 5],
        metadata: vec![0xFF, 0xFE, 0xFD],
    };

    let mut storage = Storage::default();
    owned_data.serialize_to(&mut storage, Config::default())?;

    // Deserialize with zero-copy references
    let message = Message::deserialize_from(&storage)?;
    
    // No data copying - direct buffer access!
    println!("Title: {}", message.title);      // Points into storage
    println!("Tags: {:?}", message.tags);      // Points into storage
    println!("Metadata: {:?}", message.metadata); // Points into storage

    Ok(())
}
```

## Advanced Features

### Type Interchangeability

Seamlessly convert between owned and borrowed types:

```rust
#[derive(FlatMessage)]
struct DataWriter {
    numbers: Vec<u32>,     // Owned data for writing
    text: String,
}

#[derive(FlatMessage)]
struct DataReader<'a> {
    numbers: &'a [u32],    // Zero-copy reference for reading
    text: &'a str,
}

// Serialize with Vec, deserialize as slice - automatic conversion!
```

### Version Compatibility

```rust
#[derive(FlatMessage)]
#[flat_message_options(version = 2, compatible_versions = "1,2")]
struct UserProfile {
    name: String,
    email: String,
    
    // New field - optional for backward compatibility
    age: Option<u32>,
}

// Can deserialize data from version 1 (age will be None)
// Can deserialize data from version 2 (age will be Some(value))
```

### Rich Type System

FlatMessage supports a rich type system, including:
- [x] Basic types (bool, integers, floats, strings)
- [x] Slices
- [x] Vectors
- [x] Options
- [x] Enums
- [x] Variants
- [x] Nested Structs
- [x] Timestamp
- [x] UniqueID
- [x] Versioning

```rust
#[derive(FlatMessage, Debug, Eq, PartialEq)]
struct ComplexData<'a> {
    // Basic types
    active: bool,
    score: u32,
    
    // unique message id
    id: UniqueID,

    // message timestamp
    timestamp: Timestamp,

    // Strings and vectors
    name: &'a str,
    tags: Vec<String>,
    
    // Optional fields
    description: Option<String>,
    
    // Enums and variants
    #[flat_message_item(repr = u8, kind = enum)]
    status: Status,
    #[flat_message_item(align = 1, kind = variant)]
    data: DataVariant,
    
    // Nested structures
    #[flat_message_item(align = 4, kind = struct)]
    metadata: Metadata,
}

#[derive(FlatMessageEnum, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Status {
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

#[derive(FlatMessageVariant, Debug, Eq, PartialEq)]
enum DataVariant {
    Text(String),
    Number(i64),
    Binary(Vec<u8>),
}  

#[derive(FlatMessageStruct, Debug, Eq, PartialEq)]
struct Metadata {
    author: String,
    country: String,
}

```
**Remarks:** `UniqueID` and `Timestamp` are metadata fields and can be used only once for each struct.

*More details in the [Supported Data Types](https://gdt050579.github.io/FlatMessage/chapter-2/supported_data_types.html) chapter.*

## Use Cases

FlatMessage excels in scenarios requiring:

- **High-Performance IPC**: Zero-copy communication between processes
- **Network Protocols**: Efficient serialization for network communication
- **Data Storage**: Fast serialization for databases and file systems
- **Configuration Management**: Schema-less config files with version compatibility
- **Log Processing**: High-throughput log serialization and parsing
- **Game Development**: Fast serialization for game state and networking
- **Embedded Systems**: Memory-efficient serialization with minimal overhead

## Detailed Performance Results

### Point Structure (8 bytes data)
| Library           | Size        | Ser+Deser Time | Performance |
| ----------------- | ----------- | -------------- | ----------- |
| **FlatMessage** ⚡ | 26b (+225%) | **3.76ms**     | 🥇 Fastest   |
| **FlatMessage**   | 26b (+225%) | **4.49ms**     | 🥈           |
| postcard (schema) | 3b (-63%)   | 6.95ms         |             |
| bincode (schema)  | 2b (-75%)   | 7.07ms         |             |

### Multiple Fields (210 bytes data)
| Library           | Size        | Ser+Deser Time | Performance |
| ----------------- | ----------- | -------------- | ----------- |
| **FlatMessage** ⚡ | 355b (+69%) | **23.54ms**    | 🥇 Fastest   |
| **FlatMessage**   | 355b (+69%) | **27.36ms**    | 🥈           |
| bincode (schema)  | 172b (-19%) | 39.87ms        |             |
| postcard (schema) | 154b (-27%) | 42.47ms        |             |

### Long Strings (3.9KB data)
| Library           | Size        | Ser+Deser Time | Performance |
| ----------------- | ----------- | -------------- | ----------- |
| **FlatMessage** ⚡ | 3968b (+1%) | **28.22ms**    | 🥇 Fastest   |
| postcard (schema) | 3915b (-1%) | 39.55ms        |             |
| **FlatMessage**   | 3968b (+1%) | **40.63ms**    | 🥈           |

*Results from Windows benchmarks. See [full performance results](https://gdt050579.github.io/FlatMessage/chapter-5/performance_results.html) for all platforms.*


## Documentation

- **[Complete Guide](https://gdt050579.github.io/FlatMessage/chapter-1/getting_started.html)**: Comprehensive documentation with examples
- **[API Documentation](https://docs.rs/flat_message)**: Full API reference
- **[Performance Benchmarks](https://gdt050579.github.io/FlatMessage/chapter-5/benchmarks.html)**: Detailed performance analysis
- **[Type System](https://gdt050579.github.io/FlatMessage/chapter-2/supported_data_types.html)**: All supported data types
- **[Version Management](https://gdt050579.github.io/FlatMessage/chapter-3/versioning.html)**: Compatibility and migration guide

## Zero-Copy vs Allocation

| Type           | Zero-Copy         | Memory Usage | Performance         |
| -------------- | ----------------- | ------------ | ------------------- |
| `&str`         | ✅ Yes             | Low          | Fastest             |
| `&[T]`         | ✅ Yes             | Low          | Fastest             |
| `&[u8; N]`     | ✅ Yes             | Low          | Fastest             |
| `String`       | ❌ No              | High         | Slower (allocation) |
| `Vec<T>`       | ❌ No              | High         | Slower (allocation) |
| `Option<&str>` | ✅ Yes (when Some) | Low          | Fast                |

## Requirements

- **Rust**: Version 1.70 or later (2021 edition)
- **Platforms**: Windows, macOS, Linux (32-bit and 64-bit)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with performance and developer experience in mind
- Inspired by the need for efficient, schema-less serialization in Rust
- Thanks to the Rust community for feedback and contributions

---

**Ready to supercharge your serialization?** Add FlatMessage to your project today and experience the performance difference!