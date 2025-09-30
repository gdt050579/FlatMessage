# Large Vectors

This benchmarks checks to see how well a structure containing fields of type `Option<T>` can be serialized and deserialized.
The structure is initialized as follows:
* `opt_one` is initialized to `Some("Hello, World - this is an option field")`
* `opt_two` is initialized to `Some(12345678)`
* `opt_three` is initialized to `None`
* `opt_four` is initialized to `Some([1, 2, 3, 4, 100, 200, 300, 400, 1000, 2000, 3000, 4000, 10000, 20000, 30000, 40000])`
* `opt_five` is initialized to `Some(["Hello", "World", "This", "is", "an", "option", "field"])`
* `opt_six` is initialized to `None`
* `opt_seven` is initialized to `None`


```rust
pub struct OptionFields {
    opt_one: Option<String>,
    opt_two: Option<u32>,
    opt_three: Option<bool>,
    opt_four: Option<Vec<u32>>,
    opt_five: Option<Vec<String>>,
    opt_six: Option<String>,
    opt_seven: Option<Vec<u32>>,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `145` bytes
* Protobuf: **Not Supported** (Option<T> is not supported in protobuf via prost crate)

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   4.26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.78 -   4.86]</span> |  37.45 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.92 -  44.13]</span> | **43.90** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.06 -  50.40]</span> |
| FlatMessage | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   4.52 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.81 -   5.04]</span> |  45.98 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.20 -  53.71]</span> | **50.69** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 47.01 -  56.24]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 118 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |  12.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.52 -  14.10]</span> |  48.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.57 -  53.89]</span> | **64.46** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 58.04 -  69.73]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 125 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |  11.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.40 -  13.85]</span> |  54.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 50.89 -  63.12]</span> | **70.15** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 63.19 -  79.04]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 126 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |   9.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.37 -  10.89]</span> |  62.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 57.03 -  72.54]</span> | **74.37** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.82 -  86.99]</span> |
| rmp | 188 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +29%]</span> |  10.82 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.89 -  12.04]</span> |  72.12 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.27 -  82.56]</span> | **86.56** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 82.02 -  98.82]</span> |
| json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  23.04 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.89 -  26.13]</span> | 118.43 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[110.81 - 135.33]</span> | **149.71** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[142.48 - 169.75]</span> |
| simd_json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  26.31 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.21 -  29.14]</span> | 153.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[149.29 - 205.95]</span> | **188.02** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[173.91 - 241.76]</span> |
| cbor | 187 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +28%]</span> |  26.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.49 -  30.69]</span> | 161.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[150.53 - 198.38]</span> | **190.97** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[183.57 - 335.13]</span> |
| bson | 402 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+177%]</span> |  67.99 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 63.65 -  86.69]</span> | 152.11 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[143.13 - 185.71]</span> | **232.28** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[217.78 - 269.26]</span> |
| flexbuffers | 254 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +75%]</span> | 151.98 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[145.21 - 176.35]</span> | 116.80 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[109.76 - 128.22]</span> | **282.20** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[264.99 - 322.41]</span> |
| toml | 235 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +62%]</span> | 157.57 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[143.44 - 171.29]</span> | 431.05 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[401.97 - 472.36]</span> | **601.73** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[564.37 - 655.63]</span> |
| protobuf | - | - | - | - |



### 2. MacOs Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   4.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.53 -   4.68]</span> |  20.67 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.26 -  23.96]</span> | **25.26** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.00 -  25.82]</span> |
| FlatMessage | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   4.79 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.54 -  18.56]</span> |  25.49 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.83 -  28.69]</span> | **30.17** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 29.54 -  31.25]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 125 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |  10.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.19 -  30.40]</span> |  29.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 29.11 -  30.90]</span> | **40.65** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.08 -  41.13]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 118 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |  12.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.31 -  12.49]</span> |  28.07 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.69 -  28.48]</span> | **40.99** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.18 -  41.32]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 126 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |  10.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.39 -  10.74]</span> |  37.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 36.80 -  37.78]</span> | **47.63** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 47.19 -  48.06]</span> |
| rmp | 188 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +29%]</span> |  11.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.72 -  12.11]</span> |  44.46 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 44.05 -  45.68]</span> | **56.63** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 56.12 -  56.98]</span> |
| json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  23.77 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.49 -  23.87]</span> |  68.47 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 67.91 -  68.60]</span> | **92.42** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 92.15 -  92.85]</span> |
| simd_json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  27.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.82 -  27.53]</span> | 104.36 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[103.07 - 107.31]</span> | **131.90** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[131.54 - 192.58]</span> |
| cbor | 187 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +28%]</span> |  24.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.75 -  24.39]</span> | 121.37 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[120.49 - 122.90]</span> | **146.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[145.62 - 147.81]</span> |
| bson | 402 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+177%]</span> |  52.94 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 52.37 -  53.36]</span> | 101.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[100.55 - 120.19]</span> | **159.05** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[158.11 - 160.64]</span> |
| flexbuffers | 254 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +75%]</span> | 107.38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[106.93 - 108.01]</span> |  73.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 73.58 -  74.24]</span> | **186.75** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[186.05 - 187.50]</span> |
| toml | 235 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +62%]</span> | 117.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[116.20 - 118.65]</span> | 282.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[280.15 - 286.42]</span> | **408.69** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[406.48 - 432.76]</span> |
| protobuf | - | - | - | - |


### 3. Linux Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   3.42 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.10 -   3.69]</span> |  12.64 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.20 -  14.05]</span> | **16.34** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.39 -  18.91]</span> |
| FlatMessage | 191 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +31%]</span> |   3.14 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.98 -   3.51]</span> |  18.85 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 17.88 -  21.14]</span> | **23.02** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 22.31 -  24.93]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 118 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -19%]</span> |  11.79 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 11.12 -  12.19]</span> |  22.31 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.80 -  23.84]</span> | **34.81** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.09 -  37.21]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 125 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |  10.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.10 -  11.30]</span> |  28.59 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 27.92 -  31.71]</span> | **42.14** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 41.50 -  46.19]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 126 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ -14%]</span> |   8.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  8.04 -   9.26]</span> |  34.27 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 33.12 -  37.70]</span> | **44.33** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 43.24 -  49.31]</span> |
| rmp | 188 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +29%]</span> |   9.28 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  9.05 -  10.38]</span> |  42.29 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 41.09 -  47.13]</span> | **55.92** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 54.69 -  62.79]</span> |
| json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  22.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.78 -  26.22]</span> |  73.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 71.95 -  81.41]</span> | **105.77** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 97.07 - 133.97]</span> |
| simd_json | 264 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +82%]</span> |  26.10 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.60 -  37.97]</span> |  94.13 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 88.75 -  99.57]</span> | **124.87** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[119.99 - 134.12]</span> |
| cbor | 187 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +28%]</span> |  25.62 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.91 -  27.00]</span> | 127.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[124.79 - 133.87]</span> | **154.28** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[152.52 - 165.43]</span> |
| bson | 402 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+177%]</span> |  63.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 62.09 -  66.58]</span> | 113.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[111.71 - 120.82]</span> | **186.09** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[183.00 - 244.09]</span> |
| flexbuffers | 254 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +75%]</span> | 118.33 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[113.95 - 136.28]</span> |  83.15 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 82.46 -  92.44]</span> | **230.28** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[209.08 - 262.75]</span> |
| toml | 235 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +62%]</span> | 131.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[126.57 - 164.83]</span> | 353.57 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[344.60 - 394.50]</span> | **527.41** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[500.84 - 586.29]</span> |
| protobuf | - | - | - | - |