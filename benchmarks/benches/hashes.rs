use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SMALL: &[u8] = b"A tachyon or tachyonic particle is a hypothetical particle that always travels faster than light.
Physicists believe that faster-than-light particles cannot exist because they are inconsistent with the known laws of physics.[1][2]
If such particles did exist they could be used to send signals faster than light and into the past.
According to the theory of relativity this would violate causality, leading to logical paradoxes such as the grandfather paradox.[1]
Tachyons would exhibit the unusual property of increasing in speed as their energy decreases, and would require infinite energy to slow
to the speed of light. No verifiable experimental evidence for the existence of such particles has been found.";

fn crc(input: &[u8]) -> u32 {
    crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(input)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashes");

    let big = SMALL.repeat(1_000);

    group.bench_with_input(
        BenchmarkId::new("crc_of_unknown_origins", "small"),
        &SMALL,
        |b, input| b.iter(|| common::hashes::crc32(input)),
    );
    group.bench_with_input(
        BenchmarkId::new("crc_of_unknown_origins", "big"),
        &big,
        |b, input| b.iter(|| common::hashes::crc32(input)),
    );

    group.bench_with_input(BenchmarkId::new("crc", "small"), &SMALL, |b, input| {
        b.iter(|| crc(input))
    });
    group.bench_with_input(BenchmarkId::new("crc", "big"), &big, |b, input| {
        b.iter(|| crc(input))
    });

    group.bench_with_input(BenchmarkId::new("crc_fast", "small"), &SMALL, |b, input| {
        b.iter(|| crc32fast::hash(input))
    });
    group.bench_with_input(BenchmarkId::new("crc_fast", "big"), &big, |b, input| {
        b.iter(|| crc32fast::hash(input))
    });

    group.bench_with_input(BenchmarkId::new("xxh32", "small"), &SMALL, |b, input| {
        b.iter(|| xxhash_rust::xxh32::xxh32(input, 0xFFFFFFFF))
    });
    group.bench_with_input(BenchmarkId::new("xxh32", "big"), &big, |b, input| {
        b.iter(|| xxhash_rust::xxh32::xxh32(input, 0xFFFFFFFF))
    });

    group.bench_with_input(BenchmarkId::new("xxh3_64", "small"), &SMALL, |b, input| {
        b.iter(|| xxhash_rust::xxh3::xxh3_64(input))
    });
    group.bench_with_input(BenchmarkId::new("xxh3_64", "big"), &big, |b, input| {
        b.iter(|| xxhash_rust::xxh3::xxh3_64(input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
