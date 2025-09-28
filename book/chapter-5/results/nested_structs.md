# Nested Structures

This benchmarks compares the performance of the different algorithms to serialize and deserialize a nested structure.

```rust
pub struct LevelOne {
    s1: String,
    s2: String,
    v1: u32,
    v2: u64,
    arr: Vec<u32>,
}

pub struct DepthTwo {
    name: String,
    arr: Vec<String>,
    level_1: Option<LevelOne>,    
}

pub struct NestedStrucs {
    name: String,
    protected_process: bool,
    protected_process: bool,
    level_1: Option<LevelOne>,
    level_2: Option<DepthTwo>,
}

```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `413` bytes
* Protobuf: **Supported**

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 492 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +19%]</span> |   5.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.88 -   5.45]</span> |  72.86 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 70.06 -  75.31]</span> | **79.79** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 76.33 -  82.42]</span> |
| FlatMessage | 492 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +19%]</span> |   5.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.12 -   6.01]</span> |  77.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 74.28 -  78.73]</span> | **83.24** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 79.34 -  84.23]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 363 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -13%]</span> |  12.65 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.78 -  13.56]</span> |  78.92 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 74.58 -  87.94]</span> | **92.35** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 87.00 -  97.69]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 367 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -12%]</span> |  10.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.64 -  10.96]</span> |  81.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 76.76 -  83.54]</span> | **92.47** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 88.25 -  95.42]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 374 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -10%]</span> |  10.91 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.37 -  11.46]</span> |  93.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 89.82 -  97.09]</span> | **114.19** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[106.86 - 119.30]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 382 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  16.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.90 -  18.39]</span> | 115.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[109.60 - 118.23]</span> | **136.54** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[130.97 - 150.26]</span> |
| rmp | 462 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +11%]</span> |  13.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.67 -  13.72]</span> | 114.01 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[108.96 - 117.29]</span> | **136.85** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[129.60 - 140.75]</span> |
| json | 550 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +33%]</span> |  42.16 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.85 -  44.91]</span> | 183.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[175.98 - 201.92]</span> | **232.73** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[225.25 - 252.79]</span> |
| cbor | 463 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +12%]</span> |  34.32 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 32.90 -  35.54]</span> | 214.70 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[204.56 - 223.01]</span> | **254.37** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[246.49 - 262.38]</span> |
| simd_json | 550 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +33%]</span> |  37.78 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 36.30 -  68.77]</span> | 208.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[197.99 - 212.99]</span> | **260.11** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[248.91 - 262.04]</span> |
| bson | 701 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |  67.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 64.28 -  70.99]</span> | 207.89 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[195.47 - 214.32]</span> | **293.14** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[284.69 - 301.15]</span> |
| flexbuffers | 561 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +35%]</span> | 236.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[220.19 - 248.87]</span> | 188.59 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[182.44 - 194.91]</span> | **442.59** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[421.71 - 458.80]</span> |
| toml | 568 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +37%]</span> | 443.99 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[428.89 - 507.61]</span> | 790.28 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[761.49 - 813.41]</span> | **1284.78** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1249.50 - 1332.13]</span> |


### 2. MacOs Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 492 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +19%]</span> |   7.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.08 -   7.15]</span> |  32.95 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 32.82 -  33.11]</span> | **40.11** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.08 -  40.21]</span> |
| FlatMessage | 492 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +19%]</span> |   7.09 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  7.08 -  10.23]</span> |  35.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 35.05 -  36.22]</span> | **42.32** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 42.20 -  63.22]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 367 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -12%]</span> |  10.52 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.44 -  10.55]</span> |  37.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.29 -  37.44]</span> | **48.70** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 48.40 -  48.84]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 363 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -13%]</span> |  13.09 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.08 -  13.14]</span> |  37.52 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.45 -  38.56]</span> | **50.81** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 50.73 -  69.73]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 374 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -10%]</span> |  12.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.11 -  12.24]</span> |  52.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 52.11 -  53.21]</span> | **64.47** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 64.03 -  64.99]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 382 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  14.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 14.41 -  14.87]</span> |  59.14 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.93 -  59.25]</span> | **76.91** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 76.42 -  78.01]</span> |
| rmp | 462 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +11%]</span> |  15.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.59 -  15.67]</span> |  68.62 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.49 -  69.66]</span> | **83.70** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 83.54 -  84.23]</span> |
| json | 550 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +33%]</span> |  39.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 39.25 -  39.43]</span> | 103.64 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[101.90 - 111.56]</span> | **143.19** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[142.49 - 148.35]</span> |
| simd_json | 550 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +33%]</span> |  39.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 39.55 -  40.35]</span> | 125.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[125.50 - 127.23]</span> | **168.68** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[167.92 - 169.57]</span> |
| cbor | 463 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +12%]</span> |  35.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 35.25 -  35.55]</span> | 153.95 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[153.43 - 155.27]</span> | **192.12** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[191.47 - 209.82]</span> |
| bson | 701 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +69%]</span> |  58.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.87 -  59.11]</span> | 170.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[168.88 - 173.12]</span> | **237.09** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[235.52 - 238.60]</span> |
| flexbuffers | 561 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +35%]</span> | 153.11 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[152.18 - 154.83]</span> | 112.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[112.17 - 128.58]</span> | **274.44** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[273.39 - 275.55]</span> |
| toml | 568 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +37%]</span> | 320.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[316.47 - 322.28]</span> | 517.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[514.60 - 519.24]</span> | **857.90** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[850.86 - 872.40]</span> |

### 3. Linux Execution

