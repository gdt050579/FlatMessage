# Large Vectors

This benchmarks checks to see how well a structure containing variant fields can be serialized and deserialized. The fields are instantiated with different values, as follows:
- `v1` is initialized to `MyVariant::U32(0x12345)`
- `v2` is initialized to `MyVariant::U64(0x1234567890)`
- `v3` is initialized to `MyVariant::String(String::from("Hello, World!"))`
- `v4` is initialized to `MyVariant::Vector(vec![1, 2, 3, 4, 5, 10, 20, 30, 40, 50, 100, 200, 300, 400, 500, 1000, 2000, 3000, 4000, 5000])`
- `v5` is initialized to `MyVariant::StringVector(vec![String::from("Hello"), String::from("World"), String::from("This"), String::from("is"), String::from("a"), String::from("test")])`
- `v6` is initialized to `MyVariant::SimpleVariant`
- `v7` is initialized to `MyVariant::U32(0)`
- `v8` is initialized to `MyVariant::U64(100)`
- `v9` is initialized to `None`
- `v10` is initialized to `Some(MyVariant::String(String::from("Hello, World! Testing a variant in a option field").repeat(100)))`

```rust
enum MyVariant {
    U32(u32),
    U64(u64),
    String(String),
    Vector(Vec<u32>),
    StringVector(Vec<String>),
    SimpleVariant,
}

pub struct VariantFields {
    v1: MyVariant,
    v2: MyVariant,
    v3: MyVariant,
    v4: MyVariant,
    v5: MyVariant,
    v6: MyVariant,
    v7: MyVariant,
    v8: MyVariant,
    v9: Option<MyVariant>,
    v10: Option<MyVariant>,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 50000`
* Data size: `5048` bytes
* Protobuf: **Not Supported** (directly via prost crate)

## Results

### 1. Windows Execution


### 2. MacOs Execution


### 3. Linux Execution

