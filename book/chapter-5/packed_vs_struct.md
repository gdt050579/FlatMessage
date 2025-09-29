# Packed vs Struct comparison

FlatMessage supports two types of nested structures, each with different characteristics and compatibility behaviors: `FlatMessageStruct` and `FlatMessagePacked`. To test the performance of these two types, we can use the following structure:

```rust
pub struct InnerStruct {
    s1: String,
    s2: String,
    v1: u32,
    v2: u64,
    arr: Vec<u32>,
}

pub struct NestedStruct {
    field: InnerStruct,
}
```

with the following settings (which will be used in the benchmarks):
1. For **FlatMessageStruct**:
    ```rust
    #[derive(FlatMessageStruct)]
    pub struct InnerStruct { ... }

    #[derive(FlatMessage)]
    pub struct NestedStruct {
        #[flat_message_item(kind = struct, align = 4)]
        field: InnerStruct,
    }
    ```
2. For **FlatMessagePacked**:
    ```rust
    #[derive(FlatMessagePacked)]
    pub struct InnerStruct { ... }

    #[derive(FlatMessage)]
    pub struct NestedStruct {
        #[flat_message_item(kind = packed, align = 4)]
        field: InnerStruct,
    }
    ```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 100000`
* Data size: `161` bytes


## Results

### 1. Windows Execution

### 2. MacOS Execution

### 3. Linux Execution