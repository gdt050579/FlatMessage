use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flat_message::*;
use serde::Serialize;

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
    metadata: flat_message::MetaData,
}

fn test_flat_message(process: &ProcessCreated, output: &mut Vec<u8>) -> usize {
    output.clear();
    process.serialize_to(output);
    output.len()
}

fn test_bson(process: &ProcessCreatedS) -> usize {
    let bson_data = bson::to_vec(&process).unwrap();
    bson_data.len()
}

fn test_cbor(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    ciborium::into_writer(process, &mut *output).unwrap();
    output.len()
}

fn test_json(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    serde_json::to_writer(&mut *output, process).unwrap();
    output.len()
}

fn test_rmp(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    rmp_serde::encode::write(output, process).unwrap();
    output.len()
}

fn test_bincode(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    bincode::serialize_into(&mut *output, process).unwrap();
    output.len()
}

pub fn criterion_benchmark(c: &mut Criterion) {
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
        metadata: flat_message::MetaDataBuilder::new()
            .timestamp(0xFEFEFEFE)
            .unique_id(0xABABABAB)
            .build(),
    };
    let mut output = Vec::new();

    let mut group = c.benchmark_group("🐱‍👤");

    group.bench_with_input(BenchmarkId::new("flat_message", "_"), &(), |b, _| {
        b.iter(|| test_flat_message(black_box(&process), black_box(&mut output)))
    });
    group.bench_with_input(BenchmarkId::new("cbor", "_"), &(), |b, _| {
        b.iter(|| test_cbor(black_box(&process_s), black_box(&mut output)))
    });
    group.bench_with_input(BenchmarkId::new("json", "_"), &(), |b, _| {
        b.iter(|| test_json(black_box(&process_s), black_box(&mut output)))
    });
    group.bench_with_input(BenchmarkId::new("bson", "_"), &(), |b, _| {
        b.iter(|| test_bson(black_box(&process_s)))
    });
    group.bench_with_input(BenchmarkId::new("rmp", "_"), &(), |b, _| {
        b.iter(|| test_rmp(black_box(&process_s), black_box(&mut output)))
    });
    group.bench_with_input(BenchmarkId::new("bincode", "_"), &(), |b, _| {
        b.iter(|| test_bincode(black_box(&process_s), black_box(&mut output)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
