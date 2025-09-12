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

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |   5.24 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.69 -   5.93]</span> |  23.26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.27 -  24.29]</span> | **30.12** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.53 -  37.23]</span> |
| FlatMessage | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |   5.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.09 -   9.37]</span> |  32.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 31.98 -  33.72]</span> | **39.84** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 36.56 -  43.92]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 4996 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -2%]</span> |   9.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.76 -  13.15]</span> |  36.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.21 -  38.92]</span> | **48.57** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 43.86 -  50.31]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5009 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -1%]</span> |   8.77 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.31 -  10.77]</span> |  40.38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.87 -  45.36]</span> | **52.20** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 47.74 -  54.20]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5075 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   8.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.28 -   9.84]</span> |  47.98 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.86 -  64.15]</span> | **60.30** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 57.04 -  62.03]</span> |
| rmp | 5106 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  10.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.55 -  12.18]</span> |  54.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 51.39 -  56.13]</span> | **68.88** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 64.08 -  70.95]</span> |
| cbor | 5109 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  24.55 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.84 -  26.80]</span> | 129.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[120.49 - 149.67]</span> | **156.10** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[145.30 - 160.28]</span> |
| bson | 5426 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +7%]</span> |  45.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 42.39 -  46.29]</span> | 114.96 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[111.40 - 120.21]</span> | **178.18** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[168.74 - 224.70]</span> |
| simd_json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> |  28.53 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.50 -  30.39]</span> | 169.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[161.40 - 176.34]</span> | **204.18** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[190.35 - 213.70]</span> |
| flexbuffers | 5259 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +4%]</span> | 135.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[128.13 - 155.44]</span> |  93.74 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 85.96 -  98.08]</span> | **240.00** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[222.26 - 260.34]</span> |
| json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 130.93 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[124.74 - 133.99]</span> | 123.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[118.38 - 135.17]</span> | **257.43** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[248.58 - 268.54]</span> |
| toml | 5216 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 703.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[684.13 - 724.79]</span> | 730.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[694.26 - 742.67]</span> | **1465.12** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1394.57 - 1499.54]</span> |
| protobuf | - | - | - | - |


### 2. MacOs Execution


### 3. Linux Execution

