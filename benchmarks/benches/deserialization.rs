use std::num::{NonZeroU64, NonZeroU8};

use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flat_message::FlatMessage;
use serde::{Deserialize, Serialize};

#[flat_message::flat_message]
struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
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

// ----------------------------------------------------------------------------

fn se_test_flat_message(process: &ProcessCreated, output: &mut Vec<u8>) {
    output.clear();
    process
        .serialize_to(output, flat_message::Config::default())
        .unwrap();
}

fn de_test_flat_message(input: &[u8]) -> ProcessCreated {
    ProcessCreated::deserialize_from(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    *output = bson::to_vec(&process).unwrap();
}

fn de_test_bson(input: &[u8]) -> ProcessCreatedS {
    bson::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_cbor(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    output.clear();
    ciborium::into_writer(process, &mut *output).unwrap();
}

fn de_test_cbor(input: &[u8]) -> ProcessCreatedS {
    ciborium::from_reader(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_json(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    output.clear();
    serde_json::to_writer(&mut *output, process).unwrap();
}

fn de_test_json(input: &[u8]) -> ProcessCreatedS {
    serde_json::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_rmp(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    output.clear();
    rmp_serde::encode::write(output, process).unwrap();
}

fn de_test_rmp(input: &[u8]) -> ProcessCreatedS {
    rmp_serde::decode::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bincode(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    output.clear();
    bincode::serialize_into(&mut *output, process).unwrap();
}

fn de_test_bincode(input: &[u8]) -> ProcessCreatedS {
    bincode::deserialize(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_flexbuffers(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    *output = flexbuffers::to_vec(process).unwrap();
}

fn de_test_flexbuffers(input: &[u8]) -> ProcessCreatedS {
    flexbuffers::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

pub fn criterion_benchmark(c: &mut Criterion) {
    let repeat = 100;
    let process = ProcessCreated {
        name: String::from("C:\\Windows\\System32\\example.exe").repeat(repeat),
        pid: 1234,
        parent_pid: 1,
        parent: String::from("C:\\Windows\\System32\\explorer.exe").repeat(repeat),
        user: String::from("Administrator").repeat(repeat),
        command_line: String::from("-help -verbose -debug -output C:\\output.txt").repeat(repeat),
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
        command_line: String::from("-help -verbose -debug -output C:\\output.txt").repeat(repeat),
        timestamp: NonZeroU64::new(0xFEFEFEFE).unwrap(),
        unique_id: NonZeroU64::new(0xABABABAB).unwrap(),
        version: NonZeroU8::new(1).unwrap(),
    };
    let mut data = Vec::new();

    let mut group = c.benchmark_group("deserialization");

    data.clear();
    se_test_flat_message(&process, &mut data);
    group.bench_with_input(BenchmarkId::new("flat_message", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_flat_message(black_box(&data))))
    });

    data.clear();
    se_test_json(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("json", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_json(black_box(&data))))
    });

    data.clear();
    se_test_bson(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("bson", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_bson(black_box(&data))))
    });

    data.clear();
    se_test_cbor(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("cbor", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_cbor(black_box(&data))))
    });

    data.clear();
    se_test_rmp(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("rmp", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_rmp(black_box(&data))))
    });

    data.clear();
    se_test_bincode(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("bincode", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_bincode(black_box(&data))))
    });

    data.clear();
    se_test_flexbuffers(&process_s, &mut data);
    group.bench_with_input(BenchmarkId::new("flexbuffers", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_flexbuffers(black_box(&data))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
