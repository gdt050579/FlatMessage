use std::num::NonZeroU8;

use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flat_message::{FlatMessage, Storage};
use serde::Serialize;
use bincode::Encode;

#[derive(FlatMessage)]
#[flat_message_options(version: 1)]
struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: u32,
    unique_id: u32,
}

#[derive(Serialize, Encode)]
struct ProcessCreatedS {
    struct_name: &'static str,
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: u32,
    unique_id: u32,
    version: NonZeroU8,
}

fn test_flat_message(process: &ProcessCreated, output: &mut Storage) -> usize {
    output.clear();
    process
        .serialize_to(output, flat_message::Config::default())
        .unwrap();
    output.len()
}

fn test_bson(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    bson::serialize_to_buffer(process, output).unwrap();
    output.len()
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

fn test_rmp_schema(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    rmp_serde::encode::write(output, process).unwrap();
    output.len()
}

fn test_rmp_schemaless(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    rmp_serde::encode::write_named(output, process).unwrap();
    output.len()
}

fn test_bincode(process: &ProcessCreatedS, output: &mut Vec<u8>) -> usize {
    output.clear();
    //bincode::serialize_into(&mut *output, process).unwrap();
    bincode::encode_into_std_write(process, &mut *output, bincode::config::standard()).unwrap();
    output.len()
}

fn test_flexbuffers(process: &ProcessCreatedS) -> usize {
    let data = flexbuffers::to_vec(process).unwrap();
    data.len()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let repeat = 100;
    let process = ProcessCreated {
        name: String::from("C:\\Windows\\System32\\example.exe").repeat(repeat),
        pid: 1234,
        parent_pid: 1,
        parent: String::from("C:\\Windows\\System32\\explorer.exe").repeat(repeat),
        user: String::from("Administrator").repeat(repeat),
        command_line: String::from("-help -verbose -debug -output C:\\output.txt").repeat(repeat),
        unique_id: 0xABABABAB_u32,  
        timestamp: 0xFEFEFEFE_u32,
    };
    let process_s = ProcessCreatedS {
        struct_name: "ProcessCreated",
        name: String::from("C:\\Windows\\System32\\example.exe").repeat(repeat),
        pid: 1234,
        parent_pid: 1,
        parent: String::from("C:\\Windows\\System32\\explorer.exe").repeat(repeat),
        user: String::from("Administrator").repeat(repeat),
        command_line: String::from("-help -verbose -debug -output C:\\output.txt").repeat(repeat),
        timestamp: 0xFEFEFEFE_u32,
        unique_id: 0xABABABAB_u32,
        version: NonZeroU8::new(1).unwrap(),
    };
    let mut output = Vec::new();
    let mut storage = Storage::default();   

    let mut group = c.benchmark_group("serialization");

    if false {
        group.bench_with_input(BenchmarkId::new("flat_message", "_"), &(), |b, _| {
            b.iter(|| test_flat_message(black_box(&process), black_box(&mut storage)))
        });
        group.bench_with_input(BenchmarkId::new("cbor", "_"), &(), |b, _| {
            b.iter(|| test_cbor(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("json", "_"), &(), |b, _| {
            b.iter(|| test_json(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("bson", "_"), &(), |b, _| {
            b.iter(|| test_bson(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("rmp_schema", "_"), &(), |b, _| {
            b.iter(|| test_rmp_schema(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("rmp_schemaless", "_"), &(), |b, _| {
            b.iter(|| test_rmp_schemaless(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("bincode", "_"), &(), |b, _| {
            b.iter(|| test_bincode(black_box(&process_s), black_box(&mut output)))
        });
        group.bench_with_input(BenchmarkId::new("flexbuffers", "_"), &(), |b, _| {
            b.iter(|| test_flexbuffers(black_box(&process_s)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
