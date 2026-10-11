use std::num::{NonZeroU64, NonZeroU8};

use bincode::{Decode, Encode};
use bson::doc;
use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flat_message::{FlatMessage, Storage};
use serde::{Deserialize, Serialize};

#[derive(FlatMessage)]
#[flat_message_options(version: 1)]
struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
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

fn se_test_flat_message(process: &ProcessCreated, output: &mut Storage) {
    output.clear();
    process
        .serialize_to(output, flat_message::Config::default())
        .unwrap();
}

fn de_test_flat_message(input: &Storage) -> ProcessCreated {
    ProcessCreated::deserialize_from(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson(process: &ProcessCreatedS, output: &mut Vec<u8>) {
    bson::serialize_to_buffer(process, output).unwrap();
}

fn de_test_bson(input: &[u8]) -> ProcessCreatedS {
    bson::deserialize_from_slice(input).unwrap()
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
    //bincode::serialize_into(&mut *output, process).unwrap();
    bincode::encode_into_std_write(process, &mut *output, bincode::config::standard()).unwrap();
}

fn de_test_bincode(input: &[u8]) -> ProcessCreatedS {
    bincode::decode_from_slice(input, bincode::config::standard())
        .unwrap()
        .0
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
        unique_id: flat_message::UniqueID::with_value(0xABABABAB),
        timestamp: flat_message::Timestamp::with_value(0xFEFEFEFE),
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
    let mut the_other_data = Storage::default();

    let mut group = c.benchmark_group("deserialization");

    data.clear();
    se_test_flat_message(&process, &mut the_other_data);
    group.bench_with_input(BenchmarkId::new("flat_message", "_"), &(), |b, _| {
        b.iter(|| black_box(de_test_flat_message(black_box(&the_other_data))))
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
