FlatMessage is a schema-less serialization/deseialization library written in Rust. It is designed to be really fast (in terms of both serialization and deserialization speed) while providing an adequate serialization size.

FlatMessage uses Zero-Copy deserialization, which means that it does not copy the data from the buffer to the deserialized object. Instead, it borrows the buffer and reads the data from it. 

To use FlatMessage, add the following code in your `Cargo.toml` file:

```toml
[dependencies]
flatmessage = "*"
```

then add `#[flat_message(...)]` attribute to your struct definition. Here is an example:

```rust
use flatmessage::*;

#[flat_message]
struct Point {
    x: f32,
    y: f32,
    name: String,
}

fn main() {
    let point = Point {
        x: 1.0,
        y: 2.0,
        name: "Point".to_string(),
    };

    let storage =  Storage::deault();
    // serialize the point
    if let Ok(()) = point.serialize_to(&mut storage, Config::default()) {
        // the point object is serialized to the output buffer
        / / to accss the memory buffer use output.as_slice()
        let output = storage.as_slice();
        // save the output buffer or send it via network or anothe comunication channel

        // deserialize the point
        if let Ok(point2) = Point::deserialize_from(&storage) {
            assert_eq!(point.x, point2.x);
            assert_eq!(point.y, point2.y);
            assert_eq!(point.name, point2.name);
        }
    }
}
```

The following types are supported by FlatMessage:
- `bool`
- `u8`, `u16`, `u32`, `u64`, `u128`
- `i8`, `i16`, `i32`, `i64`, `i128`
- `f32`, `f64`
- `String` and `&str`
- `Vec<T>` where `T` is any supported type
- `&[T]` where `T` is any supported type
- Enumerations (if they are represented on 8, 16 , 32 or 64 bits)

To add make an enumeration serializable, add the `FlatMessageEnum` attribute to the `#[derive]` attribute of the enumeration. Here is an example:

```rust
use flatmessage::*;

#[derive(FlatMessageEnum)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}
```

It is important to highlight that an enum has to be represented on 8, 16, 32 or 64 bits. If the enum is represented on 8 bits, the `#[repr(u8)]` attribute has to be added to the `#[derive]` attribute of the enum. If the enum is represented on 16 bits, the `#[repr(u16)]` attribute has to be added to the `#[derive]` attribute of the enum. The same applies to 32 and 64 bits. Aditionally, FlatMessage validates that the value of the enum is the corect one. If the value is not valid, the deserialization will fail. To do this, you have to provide values for each variant of the enum.


Once an enum is defined in this way, it can be used in a struct definition. Here is an example:

```rust
use flatmessage::*;

#[flat_message]
struct ColorPoint {
    x: f32,
    y: f32,
    #[flat_message(kind = "enum", repr = "u8")]
    color: Color,
}
```

The `kind` attribute is used to specify that the field is an enum. The `repr` attribute is used to specify the representation of the enum. The representation can be `u8`, `u16`, `u32` or `u64`. The `repr` attribute is optional. If it is not provided, the default value is `u8`.