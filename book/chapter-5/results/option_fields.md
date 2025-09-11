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


### 3. Linux Execution

