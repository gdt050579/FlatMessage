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

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |  12.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.94 -  13.48]</span> |  16.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 14.69 -  16.81]</span> | **24.15** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.66 -  24.94]</span> |
| FlatMessage | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |  13.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.35 -  13.68]</span> |  25.64 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.55 -  26.46]</span> | **33.67** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 32.72 -  34.72]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 4996 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -2%]</span> |  10.72 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.51 -  11.08]</span> |  27.78 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.40 -  29.04]</span> | **38.40** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.16 -  40.01]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5009 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -1%]</span> |   9.96 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.90 -  10.03]</span> |  30.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 30.03 -  30.73]</span> | **40.54** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.37 -  41.28]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5075 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |  10.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.39 -  10.58]</span> |  37.87 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.47 -  38.66]</span> | **48.61** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 47.58 -  68.96]</span> |
| rmp | 5106 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  11.52 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.50 -  12.02]</span> |  42.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 41.83 -  43.24]</span> | **54.39** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 53.97 -  55.11]</span> |
| cbor | 5109 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  22.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.06 -  22.58]</span> |  97.05 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 96.02 - 102.05]</span> | **120.43** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[119.48 - 124.76]</span> |
| bson | 5426 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +7%]</span> |  38.79 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.68 -  39.01]</span> |  82.71 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 81.96 - 101.91]</span> | **124.26** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[123.67 - 126.39]</span> |
| simd_json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> |  38.11 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.03 -  38.91]</span> | 120.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[118.82 - 123.70]</span> | **158.85** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[158.38 - 163.32]</span> |
| flexbuffers | 5259 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +4%]</span> |  97.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 96.15 - 101.65]</span> |  66.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 65.83 -  66.95]</span> | **167.95** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[166.90 - 174.09]</span> |
| json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 100.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[100.01 - 102.47]</span> |  85.69 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 85.15 - 101.58]</span> | **185.64** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[185.34 - 190.29]</span> |
| toml | 5216 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 778.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[774.68 - 803.87]</span> | 696.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[690.99 - 736.16]</span> | **1489.91** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1479.77 - 1747.68]</span> |
| protobuf | - | - | - | - |

### 3. Linux Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |   6.24 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.82 -   6.75]</span> |  13.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.72 -  19.85]</span> | **19.80** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.45 -  32.59]</span> |
| FlatMessage | 5196 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |   5.03 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.58 -   6.67]</span> |  22.95 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 18.64 -  26.22]</span> | **28.88** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.86 -  32.44]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 4996 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -2%]</span> |   8.70 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.02 -  14.57]</span> |  26.86 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.57 -  44.28]</span> | **36.57** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.88 -  53.06]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5009 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -1%]</span> |   9.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.75 -  12.83]</span> |  36.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 31.15 -  42.31]</span> | **48.27** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 42.28 -  69.02]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 5075 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   8.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.70 -  15.05]</span> |  41.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.08 -  66.74]</span> | **52.07** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 48.35 -  66.61]</span> |
| rmp | 5106 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |   9.28 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.91 -  11.28]</span> |  47.16 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.65 -  50.59]</span> | **59.07** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 56.07 -  85.17]</span> |
| cbor | 5109 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  24.83 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.68 -  27.00]</span> | 114.13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[108.59 - 123.74]</span> | **141.75** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[135.40 - 147.29]</span> |
| bson | 5426 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +7%]</span> |  48.16 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.99 -  57.07]</span> | 103.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 98.11 - 135.86]</span> | **158.63** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[151.21 - 208.65]</span> |
| simd_json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> |  27.01 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.51 -  28.63]</span> | 145.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[138.55 - 148.97]</span> | **180.56** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[171.26 - 204.58]</span> |
| flexbuffers | 5259 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +4%]</span> | 118.13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[114.57 - 175.30]</span> |  82.46 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 79.07 - 111.77]</span> | **212.49** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[203.93 - 226.69]</span> |
| json | 5211 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 138.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[133.35 - 150.30]</span> | 102.57 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 98.12 - 112.70]</span> | **240.33** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[231.69 - 285.45]</span> |
| toml | 5216 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 580.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[574.53 - 625.95]</span> | 634.01 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[613.55 - 684.37]</span> | **1248.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1196.76 - 1293.67]</span> |
| protobuf | - | - | - | - |