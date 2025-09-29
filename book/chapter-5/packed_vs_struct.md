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
* Serialization and deserialization repetitions / iteration: `n = 1000000`
* Data size: `161` bytes


## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessagePacked (&#9888;&#65039;) | 185 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  17.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 16.57 -  18.34]</span> | 129.99 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[123.44 - 135.90]</span> | **152.09** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[144.31 - 159.41]</span> |
| FlatMessagePacked  | 185 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  19.02 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 18.28 -  21.76]</span> | 148.50 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[137.33 - 153.57]</span> | **169.07** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[157.77 - 175.84]</span> |
| FlatMessageStruct  | 217 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +34%]</span> |  21.34 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.23 -  22.37]</span> | 155.35 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[145.38 - 163.77]</span> | **182.94** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[170.53 - 189.36]</span> |
| FlatMessageStruct (&#9888;&#65039;) | 217 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +34%]</span> |  21.22 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.83 -  21.58]</span> | 158.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[149.78 - 163.35]</span> | **190.52** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[180.28 - 194.45]</span> |


### 2. MacOS Execution

| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessagePacked (&#9888;&#65039;) | 185 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  15.75 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.34 -  17.77]</span> |  68.92 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 68.41 -  71.38]</span> | **86.73** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 86.27 -  88.73]</span> |
| FlatMessagePacked  | 185 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +14%]</span> |  15.85 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 15.31 -  28.30]</span> |  87.32 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 86.91 -  87.82]</span> | **104.72** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[104.06 - 105.26]</span> |
| FlatMessageStruct (&#9888;&#65039;) | 217 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +34%]</span> |  20.39 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 19.97 -  20.45]</span> |  95.73 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 95.34 -  96.00]</span> | **116.05** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[113.32 - 116.77]</span> |
| FlatMessageStruct  | 217 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ +34%]</span> |  20.34 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 20.07 -  20.40]</span> |  95.30 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[ 94.87 -  95.82]</span> | **116.12** <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[113.39 - 116.94]</span> |

### 3. Linux Execution