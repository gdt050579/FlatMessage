use ascii_table::{Align, AsciiTable};
use flat_message::{flat_message, FlatMessage};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{
    hint::black_box,
    num::{NonZeroU64, NonZeroU8},
    time::{Duration, Instant},
};

#[cfg(test)]
mod tests;

#[flat_message]
struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
}
impl Clone for ProcessCreated {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            pid: self.pid.clone(),
            parent_pid: self.parent_pid.clone(),
            parent: self.parent.clone(),
            user: self.user.clone(),
            command_line: self.command_line.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ProcessCreatedS {
    struct_name: String,
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: NonZeroU64,
    unique_id: NonZeroU64,
    version: NonZeroU8,
}

pub trait FlatMessageOwned: for<'de> FlatMessage<'de> {}
impl<T> FlatMessageOwned for T where T: for<'de> FlatMessage<'de> {}

// ----------------------------------------------------------------------------

fn se_test_flat_message<'a, T: FlatMessage<'a>>(process: &T, output: &mut Vec<u8>) {
    process.serialize_to(output);
}

fn de_test_flat_message<T: FlatMessageOwned>(input: &[u8]) -> T {
    T::deserialize_from(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    *output = bson::to_vec(&process).unwrap();
}

fn de_test_bson<S: DeserializeOwned>(input: &[u8]) -> S {
    bson::from_slice(&input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_cbor<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    ciborium::into_writer(process, &mut *output).unwrap();
}

fn de_test_cbor<S: DeserializeOwned>(input: &[u8]) -> S {
    ciborium::from_reader(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_json<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    serde_json::to_writer(&mut *output, process).unwrap();
}

fn de_test_json<S: DeserializeOwned>(input: &[u8]) -> S {
    serde_json::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_rmp_schema<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    rmp_serde::encode::write(output, process).unwrap();
}

fn se_test_rmp_schemaless<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    rmp_serde::encode::write_named(output, process).unwrap();
}

fn de_test_rmp<S: DeserializeOwned>(input: &[u8]) -> S {
    rmp_serde::decode::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bincode<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    bincode::serialize_into(&mut *output, process).unwrap();
}

fn de_test_bincode<S: DeserializeOwned>(input: &[u8]) -> S {
    bincode::deserialize(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_flexbuffers<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    *output = flexbuffers::to_vec(process).unwrap();
}

fn de_test_flexbuffers<S: DeserializeOwned>(input: &[u8]) -> S {
    flexbuffers::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

struct Result {
    name: String,
    time: Duration,
    time_s: String,
    size: usize,
}

const ITERATIONS: u32 = 1_000_000;

fn se_bench<T, FS: Fn(&T, &mut Vec<u8>)>(
    test_name: &str,
    x: &T,
    serialize: FS,
    vec: &mut Vec<u8>,
    results: &mut Vec<Result>,
) {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        vec.clear();
        black_box(serialize(x, vec));
        black_box(vec.len());
    }
    let time = start.elapsed();
    results.push(Result {
        name: format!("{}", test_name),
        time,
        time_s: format!("{:.2}", time.as_secs_f64() * 1000.0),
        size: vec.len(),
    });
}

fn de_bench<T, FD: Fn(&[u8]) -> T>(
    test_name: &str,
    input_name: &str,
    deserialize: FD,
    input: &[u8],
    results: &mut Vec<Result>,
) {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(deserialize(black_box(input)));
    }
    let time = start.elapsed();
    results.push(Result {
        name: format!("{}~{}", test_name, input_name),
        time,
        time_s: format!("{:.2}", time.as_secs_f64() * 1000.0),
        size: 0,
    });
}

fn bench<T, FS: Fn(&T, &mut Vec<u8>), FD: Fn(&[u8]) -> T>(
    test_name: &str,
    input_name: &str,
    x: &T,
    serialize: FS,
    deserialize: FD,
    se_results: &mut Vec<Result>,
    de_results: &mut Vec<Result>,
) {
    let vec = &mut Vec::with_capacity(4096);
    se_bench(test_name, x, serialize, vec, se_results);
    de_bench(test_name, input_name, deserialize, vec, de_results);
}

fn add_benches<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    name: &str,
    t: &T,
    s: &S,
    se_results: &mut Vec<Result>,
    de_results: &mut Vec<Result>,
) {
    // Little hack to redirect the deserialize_from to deserialize_from_unchecked
    // Just for testing, don't actually do this.
    struct Wrapper<T>(T);
    impl<'a, T: FlatMessage<'a>> FlatMessage<'a> for Wrapper<T> {
        fn metadata(&self) -> &flat_message::MetaData {
            todo!()
        }

        fn update_metada(&mut self, _: flat_message::MetaData) {
            todo!()
        }

        fn serialize_to(&self, output: &mut Vec<u8>) {
            self.0.serialize_to(output)
        }

        fn deserialize_from(input: &'a [u8]) -> std::result::Result<Self, flat_message::Error>
        where
            Self: Sized,
        {
            unsafe { Self::deserialize_from_unchecked(input) }
        }

        unsafe fn deserialize_from_unchecked(
            input: &'a [u8],
        ) -> std::result::Result<Self, flat_message::Error>
        where
            Self: Sized,
        {
            Ok(Wrapper(T::deserialize_from_unchecked(input)?))
        }
    }
    let wrapper = Wrapper(t.clone());

    bench(
        "flat_message",
        name,
        t,
        se_test_flat_message,
        de_test_flat_message,
        se_results,
        de_results,
    );
    bench(
        "flat_message_unchecked",
        name,
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        se_results,
        de_results,
    );
    bench(
        "rmp_schema",
        name,
        s,
        se_test_rmp_schema,
        de_test_rmp,
        se_results,
        de_results,
    );
    bench(
        "rmp_schemaless",
        name,
        s,
        se_test_rmp_schemaless,
        de_test_rmp,
        se_results,
        de_results,
    );
    bench(
        "bincode",
        name,
        s,
        se_test_bincode,
        de_test_bincode,
        se_results,
        de_results,
    );
    bench(
        "flexbuffers",
        name,
        s,
        se_test_flexbuffers,
        de_test_flexbuffers,
        se_results,
        de_results,
    );
    bench(
        "cbor",
        name,
        s,
        se_test_cbor,
        de_test_cbor,
        se_results,
        de_results,
    );
    bench(
        "bson",
        name,
        s,
        se_test_bson,
        de_test_bson,
        se_results,
        de_results,
    );
    bench(
        "json",
        name,
        s,
        se_test_json,
        de_test_json,
        se_results,
        de_results,
    );
}

fn print_one(input_name: &str, mode: &str, results: &mut Vec<Result>) {
    results.sort_by_key(|x| x.time);

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(100);
    ascii_table
        .column(0)
        .set_header("name")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("size (b)")
        .set_align(Align::Right);
    ascii_table
        .column(2)
        .set_header("time (ms)")
        .set_align(Align::Right);

    let mut r: Vec<[&dyn Display; 3]> = Vec::new();
    for i in results {
        r.push([&i.name, &i.size, &i.time_s]);
    }

    println!("-- {} -- {} --", input_name, mode);
    ascii_table.print(r);
}

fn do_one<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    input_name: &str,
    process: &T,
    process_s: &S,
) {
    let se_results = &mut Vec::with_capacity(16);
    let de_results = &mut Vec::with_capacity(16);
    add_benches(input_name, process, process_s, se_results, de_results);

    print_one(input_name, "se", se_results);
    print_one(input_name, "de", de_results);
}

fn main() {
    println!("iterations: {}", ITERATIONS);

    {
        let process_small = ProcessCreated {
            name: String::from("C:\\Windows\\System32\\example.exe"),
            pid: 1234,
            parent_pid: 1,
            parent: String::from("C:\\Windows\\System32\\explorer.exe"),
            user: String::from("Administrator"),
            command_line: String::from("-help -verbose -debug -output C:\\output.txt"),
            metadata: flat_message::MetaDataBuilder::new()
                .timestamp(0xFEFEFEFE)
                .unique_id(0xABABABAB)
                .build(),
        };
        let process_s_small = ProcessCreatedS {
            struct_name: "ProcessCreated".to_string(),
            name: String::from("C:\\Windows\\System32\\example.exe"),
            pid: 1234,
            parent_pid: 1,
            parent: String::from("C:\\Windows\\System32\\explorer.exe"),
            user: String::from("Administrator"),
            command_line: String::from("-help -verbose -debug -output C:\\output.txt"),
            timestamp: NonZeroU64::new(0xFEFEFEFE).unwrap(),
            unique_id: NonZeroU64::new(0xABABABAB).unwrap(),
            version: NonZeroU8::new(1).unwrap(),
        };
        do_one("small", &process_small, &process_s_small);
    }
    {
        let repeat = 100;
        let process = ProcessCreated {
            name: String::from("C:\\Windows\\System32\\example.exe").repeat(repeat),
            pid: 1234,
            parent_pid: 1,
            parent: String::from("C:\\Windows\\System32\\explorer.exe").repeat(repeat),
            user: String::from("Administrator").repeat(repeat),
            command_line: String::from("-help -verbose -debug -output C:\\output.txt")
                .repeat(repeat),
            metadata: flat_message::MetaDataBuilder::new()
                .timestamp(0xFEFEFEFE)
                .unique_id(0xABABABAB)
                .build(),
        };
        let process_s = ProcessCreatedS {
            struct_name: "ProcessCreated".to_string(),
            name: String::from("C:\\Windows\\System32\\example.exe").repeat(repeat),
            pid: 1234,
            parent_pid: 1,
            parent: String::from("C:\\Windows\\System32\\explorer.exe").repeat(repeat),
            user: String::from("Administrator").repeat(repeat),
            command_line: String::from("-help -verbose -debug -output C:\\output.txt")
                .repeat(repeat),
            timestamp: NonZeroU64::new(0xFEFEFEFE).unwrap(),
            unique_id: NonZeroU64::new(0xABABABAB).unwrap(),
            version: NonZeroU8::new(1).unwrap(),
        };
        do_one("big", &process, &process_s);
    }
}
