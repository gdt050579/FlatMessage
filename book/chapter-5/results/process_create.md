# Process Create Event

This benchmarks simulates a process create event (emitted by Windows Event Log). The idea is to see how well algorithms handle a real world scenario. The size of the event is `233` bytes (by setting up different strings and values).

```rust
pub struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: u32,
    unique_id: u32,
    memory_usage: u64,
    protected_process: bool,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `233` bytes
* Protobuf: **Supported**

## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 298 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +27%]</span> |   2.53 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.06 -   3.01]</span> |  17.06 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.59 -  23.34]</span> | **20.32** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 18.00 -  26.85]</span> |
| FlatMessage | 298 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +27%]</span> |   2.75 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  2.33 -   2.95]</span> |  20.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 18.86 -  26.08]</span> | **22.84** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 21.59 -  28.98]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 234 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +0%]</span> |   3.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  3.38 -   4.51]</span> |  20.86 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.23 -  26.15]</span> | **26.65** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.19 -  32.55]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 230 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  -2%]</span> |   5.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.32 -   5.99]</span> |  23.17 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.36 -  26.94]</span> | **29.71** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 25.19 -  34.40]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 238 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +2%]</span> |   4.47 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  4.00 -   6.65]</span> |  26.44 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 23.71 -  31.10]</span> | **30.58** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 29.34 -  39.52]</span> |
| *protobuf* <span style="font-family:monospace; opacity:0.5; font-size:0.75em">(schema)</span>| 240 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  +3%]</span> |   7.59 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.62 -   8.23]</span> |  28.76 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.06 -  33.60]</span> | **37.73** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 33.30 -  44.25]</span> |
| rmp | 334 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +43%]</span> |   6.56 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[  5.54 -   9.08]</span> |  39.26 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 38.51 -  49.63]</span> | **48.26** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 47.28 -  60.12]</span> |
| bson | 376 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +61%]</span> |  19.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.17 -  21.45]</span> |  67.24 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 54.95 -  74.92]</span> | **88.09** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 74.82 -  99.26]</span> |
| cbor | 334 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +43%]</span> |  16.38 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 13.99 -  18.86]</span> |  80.14 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.09 -  96.12]</span> | **98.78** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 83.80 - 115.44]</span> |
| json | 402 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +72%]</span> |  29.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 24.55 -  34.15]</span> | 118.07 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 93.40 - 126.61]</span> | **156.74** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[125.77 - 168.44]</span> |
| simd_json | 402 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +72%]</span> |  33.36 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 26.10 -  37.58]</span> | 126.90 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[108.99 - 147.91]</span> | **169.84** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[144.93 - 197.78]</span> |
| flexbuffers | 453 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +94%]</span> | 134.07 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[120.30 - 168.56]</span> |  66.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 60.73 -  84.08]</span> | **215.41** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[191.04 - 268.91]</span> |
| toml | 385 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +65%]</span> | 154.82 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[129.26 - 181.64]</span> | 317.42 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[271.30 - 359.40]</span> | **506.82** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[442.59 - 574.69]</span> |



### 2. MacOs Execution



### 3. Linux Execution

