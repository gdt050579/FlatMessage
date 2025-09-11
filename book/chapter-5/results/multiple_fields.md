# Multiple Fields

This benchmarks compares the performance of the different algorithms when serializing and deserializing a message with multiple fields. In particula for schema-less messages, this will show how well different message formats store the schema in the message.
Fields have different basic types (string, u32, u64, i32, i64, f32, f64, bool, u8, i8, u16, i16).

```rust
pub struct MultipleFields {
    field_of_type_string: String,
    field_of_type_u32: u32,
    field_of_type_u64: u64,
    field_of_type_i32: i32,
    field_of_type_i64: i64,
    field_of_type_f32: f32,
    field_of_type_f64: f64,
    field_of_type_bool: bool,
    field_of_type_u8: u8,
    field_of_type_i8: i8,
    field_of_type_u16: u16,
    field_of_type_i16: i16,
    second_field_of_type_string: String,
    second_field_of_type_u32: u32,
    second_field_of_type_u64: u64,
    second_field_of_type_i32: i32,
    second_field_of_type_i64: i64,
    third_field_of_type_string: String,
    third_field_of_type_u32: u32,
    third_field_of_type_u64: u64,
    third_field_of_type_i32: i32,
    third_field_of_type_i64: i64,
    fourth_field_of_type_string: String,
    fourth_field_of_type_u32: u32,
    fourth_field_of_type_u64: u64,
    fourth_field_of_type_i32: i32,
    fourth_field_of_type_i64: i64,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `210` bytes
* Protobuf: **Not supported** (due to the fields of type `i8` and `i16` that are not supported by protobuf)


## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   2.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.20 -   2.92]</span> |  20.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.33 -  22.89]</span> | **23.54** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.05 -  26.37]</span> |
| FlatMessage | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   2.77 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.27 -   3.43]</span> |  23.87 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.55 -  26.51]</span> | **27.36** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.19 -  30.78]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 172 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |  10.03 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.53 -  10.70]</span> |  29.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.72 -  31.81]</span> | **39.87** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 35.29 -  44.48]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -27%]</span> |  10.47 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.49 -  11.46]</span> |  30.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.74 -  32.77]</span> | **42.47** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.92 -  45.02]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -15%]</span> |  13.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.54 -  14.61]</span> |  38.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.48 -  42.74]</span> | **59.93** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 54.80 -  68.55]</span> |
| rmp | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+269%]</span> |  21.74 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 18.31 -  22.99]</span> |  90.83 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 78.58 -  99.07]</span> | **126.98** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[112.18 - 138.83]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+274%]</span> |  47.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 42.23 -  55.22]</span> | 169.09 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[148.57 - 182.64]</span> | **222.42** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[192.96 - 240.77]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  68.24 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.92 -  74.31]</span> | 140.85 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[122.75 - 151.39]</span> | **225.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[198.50 - 246.99]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+321%]</span> |  57.16 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 51.87 -  62.13]</span> | 164.07 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[144.80 - 178.24]</span> | **233.80** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[204.84 - 259.61]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  88.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 76.05 -  99.23]</span> | 168.27 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[151.13 - 181.42]</span> | **270.51** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[248.37 - 295.02]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+386%]</span> | 439.06 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[381.02 - 476.33]</span> | 181.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[165.53 - 196.50]</span> | **646.95** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[576.51 - 696.58]</span> |
| toml | 894 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+325%]</span> | 369.89 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[335.68 - 402.81]</span> | 856.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[754.40 - 938.28]</span> | **1254.35** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1127.43 - 1386.98]</span> |
| protobuf | - | - | - | - |


### 2. MacOs Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   3.04 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.04 -   4.01]</span> |  10.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.37 -  12.33]</span> | **13.46** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.45 -  14.61]</span> |
| FlatMessage | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   3.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.05 -   7.92]</span> |  13.27 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.18 -  17.01]</span> | **16.81** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.75 -  20.53]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 172 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |   7.65 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.64 -   7.88]</span> |  15.46 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.35 -  15.85]</span> | **23.49** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.40 -  24.18]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -27%]</span> |   9.53 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.52 -   9.60]</span> |  17.01 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.93 -  17.19]</span> | **26.71** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.66 -  26.80]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -15%]</span> |  12.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.10 -  12.32]</span> |  24.89 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.84 -  24.98]</span> | **37.36** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.26 -  37.59]</span> |
| rmp | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+269%]</span> |  21.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.10 -  21.50]</span> |  58.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.55 -  60.41]</span> | **80.55** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 80.29 -  82.82]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  59.76 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 59.59 -  61.49]</span> |  81.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 81.72 -  84.49]</span> | **142.89** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[142.56 - 147.54]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+321%]</span> |  51.69 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 51.63 -  52.08]</span> | 116.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[116.16 - 117.56]</span> | **169.88** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[169.48 - 172.46]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  63.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 63.40 -  65.63]</span> | 107.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[106.12 - 110.47]</span> | **173.02** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[171.21 - 177.92]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+274%]</span> |  42.49 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 42.45 -  43.61]</span> | 132.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[132.25 - 132.82]</span> | **176.32** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[175.69 - 176.87]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+386%]</span> | 349.96 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[348.16 - 359.48]</span> | 112.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[111.88 - 119.31]</span> | **467.53** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[466.12 - 480.27]</span> |
| toml | 894 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+325%]</span> | 288.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[283.89 - 295.04]</span> | 554.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[552.77 - 557.26]</span> | **857.29** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[855.34 - 864.52]</span> |
| protobuf | - | - | - | - |

### 3. Linux Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   2.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.66 -   3.49]</span> |   7.52 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.34 -  11.19]</span> | **10.22** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.79 -  15.88]</span> |
| FlatMessage | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |   2.67 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.34 -   3.86]</span> |  12.70 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.92 -  19.66]</span> | **14.87** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 14.51 -  21.55]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -27%]</span> |  10.31 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.99 -  12.47]</span> |  16.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.66 -  19.02]</span> | **27.22** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.14 -  32.00]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 172 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |  10.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.74 -  10.96]</span> |  17.78 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.95 -  35.14]</span> | **28.60** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.73 -  39.15]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -15%]</span> |  14.03 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.40 -  23.77]</span> |  27.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.72 -  36.25]</span> | **46.48** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.50 -  50.72]</span> |
| rmp | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+269%]</span> |  21.56 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.76 -  23.37]</span> |  69.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.10 -  75.75]</span> | **95.85** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 93.59 -  98.88]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  64.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 62.01 -  69.40]</span> | 113.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[109.06 - 127.41]</span> | **197.48** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[184.91 - 247.76]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  67.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 62.81 -  74.22]</span> | 125.92 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[117.82 - 146.19]</span> | **205.14** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[193.67 - 269.61]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+274%]</span> |  46.78 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 45.37 -  48.92]</span> | 151.96 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[147.25 - 155.70]</span> | **207.72** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[199.29 - 233.28]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+321%]</span> |  54.75 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 51.70 -  78.50]</span> | 144.14 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[139.10 - 160.18]</span> | **208.93** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[197.22 - 229.21]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+386%]</span> | 395.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[376.59 - 449.20]</span> | 153.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[150.59 - 165.30]</span> | **579.35** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[544.29 - 630.75]</span> |
| toml | 894 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+325%]</span> | 315.82 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[297.91 - 379.37]</span> | 761.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[735.18 - 812.90]</span> | **1101.13** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1050.93 - 1204.26]</span> |
| protobuf | - | - | - | - |
