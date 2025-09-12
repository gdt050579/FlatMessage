# Large Vectors

This benchmarks checks to see how well a structure containing different enum fields can be serialized and deserialized.
The enum have different sized variants (u8, u32, i64) and are instantiated with different values.

```rust
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 100,
    Cyan = 101,
    Magenta = 102,
}
#[repr(u32)]
enum Math {
    A = 1,
    B = 1000,
    C = 1000000,
    D = 1000000000,
}
#[repr(i64)]
enum Negative {
    A = 1,
    B = -1000,
    C = 1000000,
    D = -1000000000,
    E = 1000000000000,
    F = -1000000000000000,
}
pub struct EnumFields {
    col: Color,
    math: Math,
    neg: Negative,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 500000`
* Data size: `13` bytes
* Protobuf: **Not Supported** (enums can't be used directly in protobuf via prost crate)

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -77%]</span> |   2.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.70 -   2.97]</span> |   4.66 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.93 -   4.84]</span> | **6.38** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.24 -   6.54]</span> |
| FlatMessage (&#9888;&#65039;) | 51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+292%]</span> |   2.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.71 -   2.52]</span> |   5.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.17 -   7.11]</span> | **7.04** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.11 -   8.29]</span> |
| FlatMessage | 51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+292%]</span> |   2.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.44 -   2.65]</span> |   6.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.15 -   7.37]</span> | **7.10** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.09 -   8.72]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -77%]</span> |   6.80 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.79 -   8.42]</span> |   8.00 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.91 -   8.98]</span> | **17.42** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.30 -  19.38]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   5.62 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.19 -   6.69]</span> |  27.57 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.13 -  32.89]</span> | **34.42** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.33 -  40.73]</span> |
| rmp | 26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+100%]</span> |   7.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.63 -   8.74]</span> |  46.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.02 -  54.01]</span> | **53.70** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 39.64 -  62.60]</span> |
| json | 38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+192%]</span> |  33.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.57 -  37.36]</span> |  78.92 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.62 -  91.34]</span> | **112.82** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 82.15 - 127.90]</span> |
| bson | 45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+246%]</span> |  42.67 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 30.45 -  48.13]</span> | 102.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 72.41 - 113.37]</span> | **147.86** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[107.70 - 165.23]</span> |
| cbor | 26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+100%]</span> |  29.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.86 -  33.01]</span> | 158.47 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[116.82 - 182.08]</span> | **185.83** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[129.80 - 209.97]</span> |
| simd_json | 38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+192%]</span> |  36.12 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.98 -  49.41]</span> | 330.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[220.58 - 493.58]</span> | **384.26** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[243.10 - 545.29]</span> |
| flexbuffers | 44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+238%]</span> | 383.99 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[279.44 - 433.44]</span> | 101.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 75.51 - 112.54]</span> | **545.00** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[413.95 - 628.83]</span> |
| toml | 37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+184%]</span> | 469.24 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[307.22 - 492.86]</span> | 580.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[386.81 - 633.13]</span> | **1073.73** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[754.72 - 1187.96]</span> |
| protobuf | - | - | - | - |


### 2. MacOs Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -77%]</span> |   1.68 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.63 -   2.85]</span> |   2.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.88 -   2.27]</span> | **4.52** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.64 -   4.85]</span> |
| FlatMessage | 51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+292%]</span> |   1.83 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.81 -   1.85]</span> |   3.93 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.90 -   4.04]</span> | **5.09** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.00 -   5.88]</span> |
| FlatMessage (&#9888;&#65039;) | 51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+292%]</span> |   1.78 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.76 -   1.83]</span> |   2.76 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.66 -   4.52]</span> | **5.92** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.13 -   7.49]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -77%]</span> |   3.55 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.52 -   3.62]</span> |   4.12 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.96 -   4.22]</span> | **7.46** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.27 -   7.59]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   3.76 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.65 -   3.85]</span> |  16.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.87 -  16.71]</span> | **20.63** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.79 -  20.94]</span> |
| rmp | 26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+100%]</span> |   4.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.76 -   5.11]</span> |  30.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 29.55 -  30.88]</span> | **34.20** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 33.89 -  35.07]</span> |
| json | 38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+192%]</span> |  24.06 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.90 -  24.74]</span> |  36.92 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 36.70 -  37.95]</span> | **60.92** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 60.13 -  62.20]</span> |
| bson | 45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+246%]</span> |  27.89 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.38 -  28.39]</span> |  49.57 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 49.15 -  51.08]</span> | **81.31** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 80.06 -  82.29]</span> |
| cbor | 26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+100%]</span> |  17.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.26 -  17.79]</span> | 102.34 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[101.03 - 104.12]</span> | **121.39** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[119.61 - 123.23]</span> |
| simd_json | 38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+192%]</span> |  26.13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.75 -  26.77]</span> | 193.82 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[186.38 - 214.73]</span> | **220.08** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[213.59 - 253.01]</span> |
| flexbuffers | 44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+238%]</span> | 161.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[158.31 - 163.03]</span> |  67.87 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 66.92 -  69.34]</span> | **234.97** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[232.07 - 261.17]</span> |
| toml | 37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+184%]</span> | 195.04 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[192.15 - 197.56]</span> | 295.58 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[288.06 - 300.80]</span> | **515.11** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[499.74 - 539.25]</span> |
| protobuf | - | - | - | - |

### 3. Linux Execution

