mod get_size_min;
mod structures;

use crate::get_size_min::GetSize;
use ascii_table::{Align, AsciiTable};
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use flat_message::{FlatMessage, FlatMessageOwned, Storage};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Write;
use std::fs;
use std::hash::Hash;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn s(mut x: String) -> String {
    x.shrink_to_fit();
    x
}
fn v<T>(mut x: Vec<T>) -> Vec<T> {
    x.shrink_to_fit();
    x
}

#[macro_export]
macro_rules! t {
    ($n:ident) => {
        impl GetSize for $n {}
    };
}

struct TestData {
    vec: Vec<u8>,
    storage: Storage,
    times: u32,
}

// ----------------------------------------------------------------------------

fn se_test_flat_message<'a, T: FlatMessage<'a>>(process: &T, data: &mut TestData) {
    process
        .serialize_to(&mut data.storage, flat_message::Config::default())
        .unwrap();
}

fn de_test_flat_message<T: FlatMessageOwned>(data: &TestData) -> T {
    T::deserialize_from(&data.storage).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson<S: Serialize>(process: &S, data: &mut TestData) {
    //data.vec = bson::to_vec(&process).unwrap();
    bson::serialize_to_buffer(&process, &mut data.vec).unwrap();
}

fn de_test_bson<S: DeserializeOwned>(data: &TestData) -> S {
    //bson::from_slice(&data.vec).unwrap()
    bson::deserialize_from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_cbor<S: Serialize>(process: &S, data: &mut TestData) {
    ciborium::into_writer(process, &mut data.vec).unwrap();
}

fn de_test_cbor<S: DeserializeOwned>(data: &TestData) -> S {
    ciborium::from_reader(data.vec.as_slice()).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_json<S: Serialize>(process: &S, data: &mut TestData) {
    serde_json::to_writer(&mut data.vec, process).unwrap();
}

fn de_test_json<S: DeserializeOwned>(data: &TestData) -> S {
    serde_json::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_simd_json<S: Serialize>(process: &S, data: &mut TestData) {
    simd_json::serde::to_writer(&mut data.vec, process).unwrap();
}

fn de_test_simd_json<S: DeserializeOwned>(data: &TestData) -> S {
    simd_json::serde::from_reader(data.vec.as_slice()).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_rmp_schema<S: Serialize>(process: &S, data: &mut TestData) {
    rmp_serde::encode::write(&mut data.vec, process).unwrap();
}

fn se_test_rmp_schemaless<S: Serialize>(process: &S, data: &mut TestData) {
    rmp_serde::encode::write_named(&mut data.vec, process).unwrap();
}

fn de_test_rmp<S: DeserializeOwned>(data: &TestData) -> S {
    rmp_serde::decode::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bincode<S: Serialize + bincode::Encode>(process: &S, data: &mut TestData) {
    bincode::encode_into_std_write(process, &mut data.vec, bincode::config::standard()).unwrap();
    // bincode::serialize_into(&mut data.vec, process).unwrap();
}

fn de_test_bincode<S: DeserializeOwned + bincode::Decode<()>>(data: &TestData) -> S {
    let res = bincode::decode_from_slice(&data.vec, bincode::config::standard()).unwrap();
    res.0
}

// ----------------------------------------------------------------------------

fn se_test_flexbuffers<S: Serialize>(process: &S, data: &mut TestData) {
    data.vec = flexbuffers::to_vec(process).unwrap();
}

fn de_test_flexbuffers<S: DeserializeOwned>(data: &TestData) -> S {
    flexbuffers::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_postcard<S: Serialize>(process: &S, data: &mut TestData) {
    postcard::to_io(process, &mut data.vec).unwrap();
}

fn de_test_postcard<S: DeserializeOwned>(data: &TestData) -> S {
    postcard::from_bytes(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_toml<S: Serialize>(process: &S, data: &mut TestData) {
    let s = toml::to_string(process).unwrap();
    data.vec.extend_from_slice(s.as_bytes());
}

fn de_test_toml<S: DeserializeOwned>(data: &TestData) -> S {
    toml::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_protobuf<S: Serialize + prost::Message>(process: &S, data: &mut TestData) {
    process.encode(&mut data.vec).unwrap();
}

fn de_test_protobuf<S: DeserializeOwned + prost::Message + Default>(data: &TestData) -> S {
    S::decode(data.vec.as_slice()).unwrap()
}

// ----------------------------------------------------------------------------

struct TestTimes {
    min: Duration,
    max: Duration,
    median: Duration,
}
impl TestTimes {
    fn to_string(&self) -> String {
        format!(
            "{:>6.2} [{:>6.2} - {:>6.2}]",
            self.median.as_secs_f64() * 1000.0,
            self.min.as_secs_f64() * 1000.0,
            self.max.as_secs_f64() * 1000.0
        )
    }
}

struct Result {
    name: AlgoKind,
    top_test_name: TestKind,
    size: usize,
    needs_schema: bool,
    min_size: usize,
    size_repr: String,
    //
    times_se: Vec<Duration>,
    times_de: Vec<Duration>,
    times_se_de: Vec<Duration>,
    //
    time_se_ms: String,
    time_de_ms: String,
    time_se_de_ms: String,
    //
    compare_key: u128,
    not_available: bool,
}

impl Result {
    fn not_available(name: AlgoKind, top_test_name: TestKind) -> Self {
        Self {
            name,
            top_test_name,
            size: 0,
            needs_schema: false,
            min_size: 0,
            size_repr: String::new(),
            times_se: Vec::new(),
            times_de: Vec::new(),
            times_se_de: Vec::new(),
            time_se_ms: String::new(),
            time_de_ms: String::new(),
            time_se_de_ms: String::new(),
            compare_key: 0,
            not_available: true,
        }
    }
}

fn compute_test_times(times: &mut Vec<Duration>) -> TestTimes {
    // sort the times (it is assume that at least one time is present)
    times.sort();
    let min = times[0];
    let max = times[times.len() - 1];
    // for odd number of times, the median is the middle one
    // for even number of times, the median is the average of the two middle ones
    let middle = times.len() / 2;
    let median = if times.len() % 2 == 0 {
        (times[middle] + times[middle - 1]) / 2
    } else {
        times[middle]
    };
    TestTimes { min, max, median }
}

fn se_bench<T, FS: Fn(&T, &mut TestData) + Clone>(
    x: &T,
    serialize: FS,
    data: &mut TestData,
) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        data.vec.clear();
        data.storage.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
    }
    start.elapsed()
}

fn de_bench<T, FD: Fn(&TestData) -> T>(deserialize: FD, data: &TestData) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        black_box(deserialize(black_box(data)));
    }
    start.elapsed()
}

fn se_de_bench<T, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    x: &T,
    serialize: FS,
    deserialize: FD,
    data: &mut TestData,
) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        data.vec.clear();
        data.storage.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
        black_box(deserialize(black_box(data)));
    }
    start.elapsed()
}

fn bench<T: GetSize, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    top_test_name: TestKind,
    test_name: AlgoKind,
    x: &T,
    serialize: FS,
    deserialize: FD,
    needs_schema: bool,
    results: &mut Vec<Result>,
    repetition_times: u32,
) {
    let mut data = TestData {
        vec: Vec::default(),
        storage: Storage::default(),
        times: repetition_times,
    };
    let time_se = se_bench(x, serialize.clone(), &mut data);
    let time_de = de_bench(deserialize.clone(), &data);
    let time_se_de = se_de_bench(x, serialize, deserialize, &mut data);

    // check if the test has been performed before
    // if yes -> atunci updatam direct / altfel creem
    let index = results
        .iter()
        .position(|x| x.name == test_name && x.top_test_name == top_test_name);

    if let Some(index) = index {
        results[index].times_se.push(time_se);
        results[index].times_de.push(time_de);
        results[index].times_se_de.push(time_se_de);
    } else {
        results.push(Result {
            name: test_name,
            top_test_name,
            size: data.vec.len().max(data.storage.len()),
            min_size: x.get_heap_size(),
            needs_schema,
            times_se: vec![time_se],
            times_de: vec![time_de],
            times_se_de: vec![time_se_de],
            time_se_ms: String::new(),
            time_de_ms: String::new(),
            time_se_de_ms: String::new(),
            size_repr: String::new(),
            compare_key: 0,
            not_available: false,
        });
    }
}

// Little hack to redirect the deserialize_from to deserialize_from_unchecked
// Just for testing, don't actually do this.
#[derive(get_size_derive::GetSize)]
struct Wrapper<T>(T);
impl<'a, T: FlatMessage<'a>> FlatMessage<'a> for Wrapper<T> {
    fn serialize_to(
        &self,
        output: &mut Storage,
        config: flat_message::Config,
    ) -> std::result::Result<(), flat_message::Error> {
        self.0.serialize_to(output, config)
    }

    fn deserialize_from(input: &'a Storage) -> std::result::Result<Self, flat_message::Error>
    where
        Self: Sized,
    {
        unsafe { Self::deserialize_from_unchecked(input) }
    }

    unsafe fn deserialize_from_unchecked(
        input: &'a Storage,
    ) -> std::result::Result<Self, flat_message::Error>
    where
        Self: Sized,
    {
        Ok(Wrapper(T::deserialize_from_unchecked(input)?))
    }
}

fn add_benches<
    'a,
    T: FlatMessageOwned
        + Clone
        + Serialize
        + DeserializeOwned
        + GetSize
        + bincode::Encode
        + bincode::Decode<()>,
>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    repetition_times: u32,
    iteration_id: u32,
) {
    let wrapper = Wrapper(x.clone());

    macro_rules! b {
        ($name:expr, $x:expr, $se:expr, $de:expr, $needs_schema:expr) => {
            if all_algos || algos.contains(&$name) {
                bench(
                    top_test_name,
                    $name,
                    $x,
                    $se,
                    $de,
                    $needs_schema,
                    results,
                    repetition_times,
                );
            }
        };
    }

    use AlgoKind::*;
    b!(
        FlatMessage,
        x,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(
        FlatMessageUnchecked,
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(RmpSchema, x, se_test_rmp_schema, de_test_rmp, true);
    b!(RmpSchemaless, x, se_test_rmp_schemaless, de_test_rmp, false);
    b!(Bincode, x, se_test_bincode, de_test_bincode, true);
    b!(
        FlexBuffers,
        x,
        se_test_flexbuffers,
        de_test_flexbuffers,
        false
    );
    b!(Cbor, x, se_test_cbor, de_test_cbor, false);
    b!(Bson, x, se_test_bson, de_test_bson, false);
    b!(Json, x, se_test_json, de_test_json, false);
    b!(SimdJson, x, se_test_simd_json, de_test_simd_json, false);
    b!(Postcard, x, se_test_postcard, de_test_postcard, true);
    b!(Toml, x, se_test_toml, de_test_toml, false);
    // check to see if protobuf is present
    if iteration_id == 0 {
        results.push(Result::not_available(AlgoKind::Protobuf, top_test_name));
    }
}

fn add_benches_protobuf<
    'a,
    T: FlatMessageOwned
        + Clone
        + Serialize
        + DeserializeOwned
        + GetSize
        + bincode::Encode
        + bincode::Decode<()>
        + prost::Message
        + Default,
>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    repetition_times: u32,
) {
    let wrapper = Wrapper(x.clone());

    macro_rules! b {
        ($name:expr, $x:expr, $se:expr, $de:expr, $needs_schema:expr) => {
            if all_algos || algos.contains(&$name) {
                bench(
                    top_test_name,
                    $name,
                    $x,
                    $se,
                    $de,
                    $needs_schema,
                    results,
                    repetition_times,
                );
            }
        };
    }

    use AlgoKind::*;
    b!(
        FlatMessage,
        x,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(
        FlatMessageUnchecked,
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(RmpSchema, x, se_test_rmp_schema, de_test_rmp, true);
    b!(RmpSchemaless, x, se_test_rmp_schemaless, de_test_rmp, false);
    b!(Bincode, x, se_test_bincode, de_test_bincode, true);
    b!(
        FlexBuffers,
        x,
        se_test_flexbuffers,
        de_test_flexbuffers,
        false
    );
    b!(Cbor, x, se_test_cbor, de_test_cbor, false);
    b!(Bson, x, se_test_bson, de_test_bson, false);
    b!(Json, x, se_test_json, de_test_json, false);
    b!(SimdJson, x, se_test_simd_json, de_test_simd_json, false);
    b!(Postcard, x, se_test_postcard, de_test_postcard, true);
    b!(Toml, x, se_test_toml, de_test_toml, false);
    b!(Protobuf, x, se_test_protobuf, de_test_protobuf, true);
}

fn print_results_ascii_table(r: &[[&dyn Display; 7]], colums: &[(&str, Align)], _file_name: &str) {
    let mut ascii_table: AsciiTable = AsciiTable::default();
    ascii_table.set_max_width(200);

    for (i, (name, align)) in colums.iter().enumerate() {
        ascii_table.column(i).set_header(*name).set_align(*align);
    }

    ascii_table.print(r);
}

fn print_results_markdown(r: &[[&dyn Display; 7]], colums: &[(&str, Align)], file_name: &str) {
    let output = &mut String::with_capacity(4096);

    for i in colums {
        write!(output, "| {} ", i.0).unwrap();
    }
    writeln!(output, "|").unwrap();
    for _ in colums {
        write!(output, "| --- ").unwrap();
    }
    writeln!(output, "|").unwrap();

    for row in r {
        for i in row {
            write!(output, "| {} ", i).unwrap();
        }
        writeln!(output, "|").unwrap();
    }

    fs::write(file_name, output).unwrap();
}

fn print_results_mdbook(r: &[[&dyn Display; 7]], _columns: &[(&str, Align)], file_name: &str) {
    let mut output = String::with_capacity(4096);

    //writeln!(output, "| Algorithm | Size (b) | Serialization Time (ms) | Deserialization Time (ms) | Total Time (ms) |").unwrap();
    writeln!(
        output,
        "| Algorithm | Size (b) | Ser. (ms) | Deser. (ms) | Ser+Deser.(ms) |"
    )
    .unwrap();
    writeln!(output, "| ------ | -------: | ----------------------: | ------------------------: | --------------: |").unwrap();

    for row in r {
        // name
        let mut name = row[2].to_string();
        if name == "flat_message_unchecked" {
            name = "FlatMessage (&#9888;&#65039;)".to_string();
        }
        if name == "flat_message" {
            name = "FlatMessage".to_string();
        }
        if row[1].to_string() == "*" {
            write!(output, "| *{}* <span style=\"font-family:monospace; opacity:0.5; font-size:0.75em\">(schema)</span>", name).unwrap();
        } else {
            write!(output, "| {} ", name).unwrap();
        }
        // size
        write!(output, "| {} ", row[3]).unwrap();
        // se time
        write!(output, "| {} ", row[4]).unwrap();
        // de time
        write!(output, "| {} ", row[5]).unwrap();
        // total time
        let tmp = row[6].to_string();
        if tmp.contains("[") {
            let pos = tmp.chars().position(|c| c == '[').unwrap();
            let total_time = format!("**{}** {}", &tmp[..pos].trim(), &tmp[pos..]);
            write!(output, "| {} ", total_time).unwrap();
        } else {
            write!(output, "| {} ", tmp).unwrap();
        }
        writeln!(output, "|").unwrap();
    }
    output = output.replace(
        "[",
        r#"<span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>["#,
    );
    output = output.replace("]", r#"]</span>"#);
    output = output.replace("N/A", "-");
    // let mut mdbook_output = String::with_capacity(output.len());
    // let mut inside_sb = false;
    // for ch in output.chars() {
    //     match ch {
    //         '[' => {
    //             inside_sb = true;
    //             mdbook_output.push(ch);
    //         }
    //         ' ' => {
    //             if !inside_sb {
    //                 mdbook_output.push(ch);
    //             } else {
    //                 mdbook_output.push_str("&nbsp;");
    //             }
    //         }
    //         ']' => {
    //             inside_sb = false;
    //             mdbook_output.push(ch);
    //         }
    //         _ => {
    //             mdbook_output.push(ch);
    //         }
    //     }
    // }

    fs::write(file_name, output).unwrap();
}

fn print_results(
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    output: OutputType,
    file_name: &str,
) {
    // compute the times
    for result in results.iter_mut() {
        if result.not_available {
            result.compare_key = u128::MAX;
            continue;
        }
        let a = compute_test_times(&mut result.times_se);
        let b = compute_test_times(&mut result.times_de);
        let c = compute_test_times(&mut result.times_se_de);
        result.time_se_ms = a.to_string();
        result.time_de_ms = b.to_string();
        result.time_se_de_ms = c.to_string();
        result.compare_key = c.median.as_nanos();
    }

    results.sort_by(|x, y| {
        x.top_test_name
            .cmp(&y.top_test_name)
            .then(x.compare_key.cmp(&y.compare_key))
    });

    let colums = [
        ("top name", Align::Left),
        ("schema", Align::Center),
        ("name", Align::Left),
        ("size (b)", Align::Right),
        ("se time (ms)", Align::Right),
        ("de time (ms)", Align::Right),
        ("se + de time (ms)", Align::Right),
    ];

    let mut r: Vec<[&dyn Display; 7]> = Vec::new();
    let mut last = None;

    let dashes: [&dyn Display; 7] = [&"---", &"---", &"---", &"---", &"---", &"---", &"---"];

    let one_algo = if all_algos { false } else { algos.len() == 1 };
    let min_size = results[0].min_size;
    for i in results.iter_mut() {
        let current = Some(&i.top_test_name);
        if last.is_some() && last != current && !one_algo {
            r.push(dashes);
        }
        last = current;

        i.size_repr = if i.min_size > 0 {
            let proc = (i.size * 100 / i.min_size) as i32 - 100;
            if proc > 999 {
                format!("{} [>999%]", i.size)
            } else {
                format!("{} [{:>+4}%]", i.size, proc)
            }
        } else {
            format!("{} [----%]", i.size)
        };

        let ch = if i.needs_schema { &'*' } else { &' ' };
        if i.not_available {
            i.size_repr = "N/A".to_string();
            i.time_se_ms = "N/A".to_string();
            i.time_de_ms = "N/A".to_string();
            i.time_se_de_ms = "N/A".to_string();
        }
        r.push([
            i.top_test_name.display(),
            ch,
            i.name.display(),
            &i.size_repr,
            &i.time_se_ms,
            &i.time_de_ms,
            &i.time_se_de_ms,
        ]);
    }

    match output {
        OutputType::Ascii => {
            print_results_ascii_table(&r, &colums, file_name);
            println!("Min size: {min_size}");
        }
        OutputType::Markdown => {
            print_results_markdown(&r, &colums, file_name);
        }
        OutputType::Mdbook => {
            print_results_mdbook(&r, &colums, file_name);
        }
    }
}

fn do_one<
    'a,
    T: FlatMessageOwned
        + Clone
        + Serialize
        + DeserializeOwned
        + GetSize
        + bincode::Encode
        + bincode::Decode<()>,
>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    repetition_times: u32,
    iteration_id: u32,
) {
    add_benches(
        top_test_name,
        x,
        results,
        algos,
        all_algos,
        repetition_times,
        iteration_id,
    );
}

fn do_one_protobuf<
    'a,
    T: FlatMessageOwned
        + Clone
        + Serialize
        + DeserializeOwned
        + GetSize
        + bincode::Encode
        + bincode::Decode<()>
        + prost::Message
        + Default,
>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    repetition_times: u32,
) {
    add_benches_protobuf(
        top_test_name,
        x,
        results,
        algos,
        all_algos,
        repetition_times,
    );
}

macro_rules! tests {
    ($enum_name:ident, $(($name:literal, $v:ident)),+) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
        enum $enum_name {
            $(
                $v,
            )+
        }
        impl $enum_name {
            // fn name(self) -> &'static str {
            //     match self {
            //         $(
            //             Self::$v => $name,
            //         )+
            //     }
            // }
            fn display(self) -> &'static dyn Display {
                match self {
                    $(
                        Self::$v => &$name,
                    )+
                }
            }
            fn all() -> &'static [&'static str] {
                &[
                    $(
                        $name,
                    )+
                ]
            }
        }
        impl From<&str> for $enum_name {
            fn from(value: &str) -> Self {
                match value {
                    $(
                        $name => Self::$v,
                    )+
                    _ => panic!("unknown option: {}\navailable option: {}", value, Self::all().join(", ")),
                }
            }
        }
    };
}

tests! {
    TestKind,
    ("process_create", ProcessCreate),
    ("long_strings", LongStrings),
    ("point", Point),
    ("multiple_fields", MultipleFields),
    ("multiple_integers", MultipleIntegers),
    ("multiple_bools", MultipleBools),
    ("vectors", Vectors),
    ("large_vectors", LargeVectors),
    ("enum_fields", EnumFields),
    ("enum_lists", EnumLists),
    ("small_enum_lists", SmallEnumLists),
    ("strings_lists", StringLists),
    ("one_bool", OneBool),
    ("option_fields", OptionFields)
}

tests! {
    AlgoKind,
    ("flat_message", FlatMessage),
    ("flat_message_unchecked", FlatMessageUnchecked),
    ("rmp", RmpSchema),
    ("rmp", RmpSchemaless),
    ("bincode", Bincode),
    ("flexbuffers" , FlexBuffers),
    ("cbor", Cbor),
    ("bson", Bson),
    ("json", Json),
    ("simd_json", SimdJson),
    ("postcard", Postcard),
    ("toml", Toml),
    ("protobuf", Protobuf)
}

fn split_tests<'x, T>(input: &'x str) -> (bool, HashSet<T>)
where
    T: From<&'x str> + Eq + Hash,
{
    if input == "all" {
        (true, HashSet::new())
    } else {
        let tests = input.split(',').map(|x| T::from(x)).collect();
        (false, tests)
    }
}

#[derive(Subcommand, Debug, Default)]
#[command(subcommand_precedence_over_arg = true)]
enum Commands {
    Run,
    #[default]
    Help,
    ListAlgos,
    ListTests,
    MDBookTests,
}

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
enum OutputType {
    #[default]
    Ascii,
    Markdown,
    Mdbook,
}

#[derive(clap::Parser)]
#[command(
    name = "benchmarks",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value_t = 1_000_000, global = true)]
    times: u32,
    #[arg(long, default_value = "all", global = true)]
    tests: String,
    #[arg(long, default_value = "all", global = true)]
    algos: String,
    #[arg(long, default_value_t = false, global = true)]
    names: bool,
    #[arg(long, default_value_t = 10, global = true)]
    iterations: u32,
    #[arg(long, value_enum, default_value_t = OutputType::Ascii, global = true)]
    output: OutputType,
    #[arg(long, default_value = "result.md", global = true)]
    file_name: String,
}

fn run_tests(args: Args, test_name: &str) {
    let (all_tests, tests) = split_tests::<TestKind>(&args.tests);
    let (all_algos, algos) = split_tests(&args.algos);

    let results = &mut Vec::new();
    macro_rules! run {
        ($name:expr, $x:expr, $iteration_id:expr) => {
            if all_tests || tests.contains(&$name) {
                do_one(
                    $name,
                    $x,
                    results,
                    &algos,
                    all_algos,
                    args.times,
                    $iteration_id,
                );
            }
        };
    }
    macro_rules! run_protobuf {
        ($name:expr, $x:expr) => {
            if all_tests || tests.contains(&$name) {
                do_one_protobuf($name, $x, results, &algos, all_algos, args.times);
            }
        };
    }

    println!("Starting execution of test: {}", test_name);
    use std::io::{self, Write};
    use TestKind::*;
    for i in 0..args.iterations {
        print!("- Running iteration {:>2}/{}", i + 1, args.iterations);
        io::stdout().flush().unwrap();
        let start = Instant::now();
        {
            let s = structures::process_create::generate();
            run_protobuf!(ProcessCreate, &s);
        }
        {
            let s = structures::long_strings::generate(100);
            run_protobuf!(LongStrings, &s);
        }
        {
            let s = structures::point::generate();
            run_protobuf!(Point, &s);
        }
        {
            let s = structures::one_bool::generate();
            run_protobuf!(OneBool, &s);
        }
        {
            let s = structures::multiple_fields::generate();
            run!(MultipleFields, &s, i);
        }
        {
            let s = structures::multiple_integers::generate();
            run_protobuf!(MultipleIntegers, &s);
        }
        {
            let s = structures::vectors::generate();
            run!(Vectors, &s, i);
        }
        {
            let s = structures::large_vectors::generate();
            run_protobuf!(LargeVectors, &s);
        }
        {
            let s = structures::enum_fields::generate();
            run!(EnumFields, &s, i);
        }
        {
            let s = structures::enum_lists::generate();
            run!(EnumLists, &s, i);
        }
        {
            let s = structures::small_enum_lists::generate();
            run!(SmallEnumLists, &s, i);
        }
        {
            let s = structures::multiple_bools::generate();
            run_protobuf!(MultipleBools, &s);
        }
        {
            let s = structures::string_lists::generate();
            run!(StringLists, &s, i);
        }
        {
            let s = structures::option_fields::generate();
            run!(OptionFields, &s, i);
        }
        println!(" done in {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    }

    print_results(results, &algos, all_algos, args.output, &args.file_name);
}

fn run_one_mdbook_test(test_name: &str, test_filter: &str, times: u32) {
    if test_filter != "all" && !test_filter.contains(test_name) {
        return;
    }
    let a = Args {
        tests: test_name.to_string(),
        algos: "all".to_string(),
        times,
        iterations: 10,
        output: OutputType::Mdbook,
        names: false,
        command: Commands::Run,
        file_name: format!("mdbook_{}.md", test_name),
    };
    run_tests(a, test_name);
}
fn run_mdbook_tests(test_filter: &str) {
    run_one_mdbook_test("multiple_fields", test_filter,100_000);
    run_one_mdbook_test("point", test_filter, 500_000);
    run_one_mdbook_test("long_strings", test_filter, 100_000); 
    run_one_mdbook_test("large_vectors", test_filter, 100); 
    run_one_mdbook_test("enum_fields", test_filter, 500_000);
    run_one_mdbook_test("option_fields", test_filter, 100_000);
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Run => {
            run_tests(args, "*");
        }
        Commands::Help => {
            println!("usage: {} <command>", env!("CARGO_BIN_NAME"));
        }
        Commands::ListAlgos => {
            println!("available algos: {}", AlgoKind::all().join(", "));
        }
        Commands::ListTests => {
            println!("available tests: {}", TestKind::all().join(", "));
        }
        Commands::MDBookTests => run_mdbook_tests(&args.tests),
    }
}
