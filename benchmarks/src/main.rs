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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Mode {
    Serialize,
    Deserialize,
}

struct Result {
    name: &'static str,
    top_test_name: &'static str,
    time: Duration,
    time_s: String,
    size: usize,
    mode: Mode,
}

const ITERATIONS: u32 = 1_000_000;

fn se_bench<T, FS: Fn(&T, &mut Vec<u8>)>(
    top_test_name: &'static str,
    test_name: &'static str,
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
        name: test_name,
        top_test_name,
        time,
        time_s: format!("{:.2}", time.as_secs_f64() * 1000.0),
        size: vec.len(),
        mode: Mode::Serialize,
    });
}

fn de_bench<T, FD: Fn(&[u8]) -> T>(
    top_test_name: &'static str,
    test_name: &'static str,
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
        name: test_name,
        top_test_name,
        time,
        time_s: format!("{:.2}", time.as_secs_f64() * 1000.0),
        size: 0,
        mode: Mode::Deserialize,
    });
}

fn bench<T, FS: Fn(&T, &mut Vec<u8>), FD: Fn(&[u8]) -> T>(
    top_test_name: &'static str,
    test_name: &'static str,
    x: &T,
    serialize: FS,
    deserialize: FD,
    results: &mut Vec<Result>,
) {
    let vec = &mut Vec::with_capacity(4096);
    se_bench(top_test_name, test_name, x, serialize, vec, results);
    de_bench(top_test_name, test_name, deserialize, vec, results);
}

fn add_benches<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    top_test_name: &'static str,
    t: &T,
    s: &S,
    results: &mut Vec<Result>,
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
        top_test_name,
        "flat_message",
        t,
        se_test_flat_message,
        de_test_flat_message,
        results,
    );
    bench(
        top_test_name,
        "flat_message_unchecked",
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        results,
    );
    bench(
        top_test_name,
        "rmp_schema",
        s,
        se_test_rmp_schema,
        de_test_rmp,
        results,
    );
    bench(
        top_test_name,
        "rmp_schemaless",
        s,
        se_test_rmp_schemaless,
        de_test_rmp,
        results,
    );
    bench(
        top_test_name,
        "bincode",
        s,
        se_test_bincode,
        de_test_bincode,
        results,
    );
    bench(
        top_test_name,
        "flexbuffers",
        s,
        se_test_flexbuffers,
        de_test_flexbuffers,
        results,
    );
    bench(
        top_test_name,
        "cbor",
        s,
        se_test_cbor,
        de_test_cbor,
        results,
    );
    bench(
        top_test_name,
        "bson",
        s,
        se_test_bson,
        de_test_bson,
        results,
    );
    bench(
        top_test_name,
        "json",
        s,
        se_test_json,
        de_test_json,
        results,
    );
}

fn print_results(results: &mut Vec<Result>) {
    results.sort_by(|x, y| {
        x.top_test_name
            .cmp(&y.top_test_name)
            .then(x.mode.cmp(&y.mode).then(x.time.cmp(&y.time)))
    });

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(100);
    ascii_table
        .column(0)
        .set_header("mode")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("top name")
        .set_align(Align::Left);
    ascii_table
        .column(2)
        .set_header("name")
        .set_align(Align::Left);
    ascii_table
        .column(3)
        .set_header("size (b)")
        .set_align(Align::Right);
    ascii_table
        .column(4)
        .set_header("time (ms)")
        .set_align(Align::Right);

    let mut r: Vec<[&dyn Display; 5]> = Vec::new();
    let mut last = None;
    for i in results {
        let current = Some((i.top_test_name, i.mode));
        if !last.is_none() && last != current {
            r.push([&"---", &"---", &"---", &"---", &"---"]);
        }
        last = current;

        let mode: &dyn Display = match i.mode {
            Mode::Serialize => &"s",
            Mode::Deserialize => &"d",
        };
        r.push([mode, &i.top_test_name, &i.name, &i.size, &i.time_s]);

    }

    ascii_table.print(r);
}

fn do_one<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    top_test_name: &'static str,
    process: &T,
    process_s: &S,
    results: &mut Vec<Result>,
) {
    add_benches(top_test_name, process, process_s, results);
}

fn main() {
    println!("iterations: {}", ITERATIONS);
    let results = &mut Vec::new();
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
        do_one("small", &process_small, &process_s_small, results);
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
        do_one("big", &process, &process_s, results);
    }

    print_results(results);
}
