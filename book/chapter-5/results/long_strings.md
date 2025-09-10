# Long Strings

This benchmarks compares the performance of the different algorithms when dealing with a structure that contains multiple long strings.
Each of the string will be instantiated with large strings (between 100 and 2000 characters) making the total size of the structure close to 4000 bytes.
    
```rust
pub struct LongStringStructure {
    string_one: String,
    string_two: String,
    string_three: String,
    string_four: String,
    value_one: u32,
    value_two: u64,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `3919` bytes
* Protobuf: **Supported**

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 3968 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |   7.32 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  6.31 -   8.86]</span> |  20.41 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.69 -  27.33]</span> | **28.22** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.88 -  35.43]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3915 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -1%]</span> |   6.56 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.96 -   9.44]</span> |  33.91 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 31.59 -  41.96]</span> | **39.55** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 37.61 -  44.44]</span> |
| FlatMessage | 3968 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |   6.65 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.92 -   9.10]</span> |  34.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 32.12 -  42.88]</span> | **40.63** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 35.66 -  49.72]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3920 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   6.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.56 -   8.26]</span> |  36.49 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.02 -  43.98]</span> | **43.08** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.01 -  52.90]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3922 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   5.89 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.33 -   8.05]</span> |  36.31 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 34.14 -  46.12]</span> | **43.21** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.46 -  56.13]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 3921 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   7.01 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  6.50 -   8.47]</span> |  36.34 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 33.69 -  43.30]</span> | **43.76** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 40.96 -  52.60]</span> |
| rmp | 3989 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |   6.71 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  6.22 -   9.30]</span> |  45.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 41.83 -  59.64]</span> | **54.31** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 51.52 -  69.65]</span> |
| bson | 4014 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |  13.09 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 12.06 -  17.22]</span> |  54.05 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 50.04 -  66.43]</span> | **71.57** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.23 -  85.13]</span> |
| cbor | 3989 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +1%]</span> |  11.26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 10.88 -  15.10]</span> |  78.63 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 77.29 -  99.97]</span> | **91.99** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 88.11 - 120.60]</span> |
| flexbuffers | 4041 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> | 107.08 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[101.70 - 128.89]</span> |  66.68 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 64.52 -  77.82]</span> | **185.35** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[180.24 - 219.79]</span> |
| simd_json | 4011 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |  27.61 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.73 -  28.45]</span> | 207.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[195.33 - 236.90]</span> | **246.34** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[231.25 - 276.01]</span> |
| json | 4011 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> | 179.69 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[170.16 - 207.78]</span> | 113.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[104.95 - 122.49]</span> | **298.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[283.15 - 316.62]</span> |
| toml | 4010 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> | 821.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[786.00 - 907.08]</span> | 758.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[713.60 - 796.55]</span> | **1638.54** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[1545.04 - 1743.35]</span> |



### 2. MacOs Execution

