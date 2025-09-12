# Large Vectors

This benchmarks compares the performance of the different algorithms when dealing with a structure that contains multiple large vectors.
The vectors will be instantiated as follows:
* ints: 2000 elements between 200 and 220
* floats: 10000 elements between -1000000.0 and 1000000.0
* uints: 25000 elements between 0 and 1000000
* doubles: 30000 elements between 0.0 and 1000000.0

```rust
pub struct LargeVectors {
    ints: Vec<i32>,
    floats: Vec<f32>,
    uints: Vec<u32>,
    doubles: Vec<f64>,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100`
* Data size: `388008` bytes
* Protobuf: **Supported**

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage | 388060 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   1.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.31 -   2.61]</span> |   1.32 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.12 -  13.24]</span> | **2.65** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.28 -  13.78]</span> |
| FlatMessage (&#9888;&#65039;) | 388060 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   1.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.15 -   2.55]</span> |   1.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  1.00 -  14.00]</span> | **2.68** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.37 -  12.95]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 407012 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +4%]</span> |  15.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.72 -  25.95]</span> |  14.02 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.95 -  23.81]</span> | **29.96** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.90 -  46.78]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 358260 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  20.11 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.25 -  24.88]</span> |  10.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.43 -  15.52]</span> | **30.74** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.41 -  42.54]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 358265 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  17.05 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 14.16 -  39.22]</span> |  15.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.50 -  29.45]</span> | **32.28** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.52 -  64.30]</span> |
| rmp | 445039 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  16.85 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 14.12 -  33.93]</span> |  27.71 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.07 -  48.36]</span> | **45.39** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.31 -  75.90]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 445013 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  16.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.82 -  32.05]</span> |  27.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.33 -  52.29]</span> | **45.46** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 39.83 -  91.06]</span> |
| cbor | 320339 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -18%]</span> |  49.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 43.10 -  64.13]</span> |  94.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 84.26 - 111.31]</span> | **145.84** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[131.47 - 170.92]</span> |
| flexbuffers | 264099 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -32%]</span> |  66.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.72 -  98.01]</span> | 110.55 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 97.93 - 156.83]</span> | **180.88** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[157.36 - 243.98]</span> |
| json | 539243 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +38%]</span> | 265.55 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[227.27 - 308.44]</span> |  99.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 94.64 - 116.47]</span> | **353.35** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[330.89 - 417.55]</span> |
| bson | 960615 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+147%]</span> | 190.55 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[157.87 - 219.51]</span> | 244.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[204.61 - 285.33]</span> | **432.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[371.42 - 504.68]</span> |
| simd_json | 539243 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +38%]</span> | 286.91 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[262.48 - 337.45]</span> | 177.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[167.55 - 199.11]</span> | **482.78** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[433.14 - 559.85]</span> |
| toml | 606238 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +56%]</span> | 350.72 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[314.07 - 397.58]</span> | 1259.97 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1051.49 - 1489.37]</span> | **1622.38** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1398.75 - 2618.01]</span> |


### 2. MacOs Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 388060 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   0.93 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  0.89 -   1.03]</span> |   0.71 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  0.67 -   0.92]</span> | **2.70** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.45 -   2.81]</span> |
| FlatMessage | 388060 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   0.96 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  0.93 -   1.13]</span> |   0.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  0.68 -   0.83]</span> | **2.87** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.69 -   2.94]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 358260 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  13.56 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.41 -  14.09]</span> |   6.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  6.19 -   8.04]</span> | **19.89** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.85 -  22.84]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 407012 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +4%]</span> |   9.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.21 -   9.53]</span> |  10.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.63 -  14.85]</span> | **21.19** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.81 -  23.30]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 358265 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -8%]</span> |  10.77 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.75 -  11.20]</span> |  19.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.00 -  20.37]</span> | **30.52** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.59 -  31.33]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 445013 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  17.62 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.35 -  17.97]</span> |  31.95 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 31.46 -  33.94]</span> | **50.05** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 49.24 -  54.93]</span> |
| rmp | 445039 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  17.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.31 -  20.36]</span> |  33.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 31.54 -  37.02]</span> | **51.29** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 49.43 -  54.87]</span> |
| flexbuffers | 264099 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -32%]</span> |  60.09 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.93 -  61.57]</span> |  61.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 59.33 -  67.41]</span> | **125.72** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[123.06 - 134.17]</span> |
| cbor | 320339 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -18%]</span> |  46.99 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 46.91 -  48.50]</span> |  81.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 80.76 -  83.73]</span> | **128.18** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[127.71 - 153.43]</span> |
| bson | 960615 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+147%]</span> | 118.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[118.05 - 122.29]</span> | 153.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[149.42 - 157.42]</span> | **274.78** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[266.38 - 295.11]</span> |
| json | 539243 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +38%]</span> | 194.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[193.23 - 231.19]</span> |  82.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 78.61 -  85.68]</span> | **280.23** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[272.33 - 286.06]</span> |
| simd_json | 539243 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +38%]</span> | 214.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[212.87 - 247.29]</span> |  98.83 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 95.00 - 120.02]</span> | **317.99** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[313.92 - 326.43]</span> |
| toml | 606238 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +56%]</span> | 202.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[200.94 - 213.92]</span> | 672.79 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[658.82 - 697.97]</span> | **873.71** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[862.31 - 902.96]</span> |

### 3. Linux Execution

