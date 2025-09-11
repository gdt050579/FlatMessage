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


### 2. MacOs Execution


### 3. Linux Execution

