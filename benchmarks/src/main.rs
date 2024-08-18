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

fn test_flat_message(process: &ProcessCreated, output: &mut Vec<u8>) {
    process.serialize_to(output);
}

fn test_bson<'x>(process: &ProcessCreatedS, output: &'x mut Vec<u8>) {
    *output = bson::to_vec(&process).unwrap();
}

fn test_cbor(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    ciborium::into_writer(process, &mut *output).unwrap();
}

fn test_json(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    serde_json::to_writer(&mut *output, process).unwrap();
}

fn test_rmp(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    rmp_serde::encode::write(output, process).unwrap();
}

fn test_bincode(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    bincode::serialize_into(&mut *output, process).unwrap();
}

fn test_flexbuffers(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    *output = flexbuffers::to_vec(process).unwrap();
}

struct Result {
    name: &'static str,
    time: Duration,
    time_s: String,
    size: usize,
}

const ITERATIONS: u32 = 1_000_000;

fn bench<T, F: Fn(&T, &mut Vec<u8>)>(name: &'static str, x: &T, f: F, results: &mut Vec<Result>) {
    let mut vec = Vec::with_capacity(4096);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        vec.clear();
        black_box(f(x, &mut vec));
        black_box(vec.len());
    }
    let time = start.elapsed();
    results.push(Result {
        name,
        time,
        time_s: format!("{:?}", time),
        size: vec.len(),
    });
}

fn add_benches(process: &ProcessCreated, process_s: &ProcessCreatedS, results: &mut Vec<Result>) {
    bench("flat_message", process, test_flat_message, results);
    bench("cbor", process_s, test_cbor, results);
    bench("bson", process_s, test_bson, results);
    bench("json", process_s, test_json, results);
    bench("rmp", process_s, test_rmp, results);
    bench("bincode", process_s, test_bincode, results);
    bench("flexbuffers", process_s, test_flexbuffers, results);
}

fn main() {
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
    let results = &mut Vec::with_capacity(16);
    add_benches(&process, &process_s, results);

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
        .set_header("time")
        .set_align(Align::Right);

    let mut r: Vec<[&dyn Display; 3]> = Vec::new();
    for i in results {
        r.push([&i.name, &i.size, &i.time_s]);
    }

    println!("iterations: {}", ITERATIONS);
    ascii_table.print(r);
}
