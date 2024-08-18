use ascii_table::{Align, AsciiTable};
use flat_message::{flat_message, FlatMessage};
use serde::Serialize;
use std::fmt::Display;
use std::{
    hint::black_box,
    num::{NonZeroU64, NonZeroU8},
    time::{Duration, Instant},
};

#[flat_message]
struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
}

#[derive(Serialize)]
struct ProcessCreatedS {
    struct_name: &'static str,
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

fn test_flat_message<T: FlatMessage>(process: &T, output: &mut Vec<u8>) {
    process.serialize_to(output);
}

fn test_bson<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    *output = bson::to_vec(&process).unwrap();
}

fn test_cbor<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    ciborium::into_writer(process, &mut *output).unwrap();
}

fn test_json<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    serde_json::to_writer(&mut *output, process).unwrap();
}

fn test_rmp_schema<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    rmp_serde::encode::write(output, process).unwrap();
}

fn test_rmp_schemaless<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    rmp_serde::encode::write_named(output, process).unwrap();
}

fn test_bincode<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    bincode::serialize_into(&mut *output, process).unwrap();
}

fn test_flexbuffers<S: Serialize>(process: &S, output: &mut Vec<u8>) {
    *output = flexbuffers::to_vec(process).unwrap();
}

struct Result {
    name: String,
    time: Duration,
    time_s: String,
    size: usize,
}

const ITERATIONS: u32 = 1_000_000;

fn bench<T, F: Fn(&T, &mut Vec<u8>)>(
    test_name: &str,
    input_name: &str,
    x: &T,
    f: F,
    results: &mut Vec<Result>,
) {
    let mut vec = Vec::with_capacity(4096);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        vec.clear();
        black_box(f(x, &mut vec));
        black_box(vec.len());
    }
    let time = start.elapsed();
    results.push(Result {
        name: format!("{}~{}", test_name, input_name),
        time,
        time_s: format!("{:.2}", time.as_secs_f64() * 1000.0),
        size: vec.len(),
    });
}

fn add_benches<T: FlatMessage, S: Serialize>(
    input_name: &str,
    process: &T,
    process_s: &S,
    results: &mut Vec<Result>,
) {
    bench(
        "flat_message",
        input_name,
        process,
        test_flat_message,
        results,
    );
    bench("cbor", input_name, process_s, test_cbor, results);
    bench("bson", input_name, process_s, test_bson, results);
    bench("json", input_name, process_s, test_json, results);
    bench(
        "rmp_schema",
        input_name,
        process_s,
        test_rmp_schema,
        results,
    );
    bench(
        "rmp_schemaless",
        input_name,
        process_s,
        test_rmp_schemaless,
        results,
    );
    bench("bincode", input_name, process_s, test_bincode, results);
    bench(
        "flexbuffers",
        input_name,
        process_s,
        test_flexbuffers,
        results,
    );
}

fn do_one<T: FlatMessage, S: Serialize>(input_name: &str, process: &T, process_s: &S) {
    let results = &mut Vec::with_capacity(16);
    add_benches(input_name, process, process_s, results);
    results.sort_by_key(|x| x.time);

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(50);
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

    ascii_table.print(r);
}

fn main() {
    println!("iterations: {}", ITERATIONS);

    {
        let process = ProcessCreated {
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
        let process_s = ProcessCreatedS {
            struct_name: "ProcessCreated",
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
        do_one("small", &process, &process_s);
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
            struct_name: "ProcessCreated",
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
