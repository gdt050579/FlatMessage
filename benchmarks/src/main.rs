use ascii_table::{Align, AsciiTable};
use clap::Parser;
use flat_message::{FlatMessage, FlatMessageOwned, Storage, VecLike};
use rkyv::ser::serializers::{AlignedSerializer, BufferScratch, CompositeSerializer};
use rkyv::ser::Serializer;
use rkyv::{AlignedVec, Infallible};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

mod structures;
#[cfg(test)]
mod tests;

struct TestData {
    vec: Vec<u8>,
    storage: Storage,
    rykv_buffer: AlignedVec,
    rykv_scratch: AlignedVec,
    iterations: u32,
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
    data.vec = bson::to_vec(&process).unwrap();
}

fn de_test_bson<S: DeserializeOwned>(data: &TestData) -> S {
    bson::from_slice(&data.vec).unwrap()
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

fn se_test_bincode<S: Serialize>(process: &S, data: &mut TestData) {
    bincode::serialize_into(&mut data.vec, process).unwrap();
}

fn de_test_bincode<S: DeserializeOwned>(data: &TestData) -> S {
    bincode::deserialize(&data.vec).unwrap()
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

type RykvS<'x> = CompositeSerializer<AlignedSerializer<&'x mut AlignedVec>, BufferScratch<&'x mut AlignedVec>>;

fn se_test_rykv<'x, __S, T>(x: &T, data: &'x mut TestData)
where
    __S: rkyv::Fallible + ?Sized,
    T: serde::Serialize + rkyv::Serialize<RykvS<'x>> + rkyv::Fallible,
{
    let mut serializer: RykvS = CompositeSerializer::new(
        AlignedSerializer::new(&mut data.rykv_buffer),
        BufferScratch::new(&mut data.rykv_scratch),
        Infallible,
    );
    serializer.serialize_value(x).unwrap();
}

fn de_test_rykv<S: DeserializeOwned>(data: &TestData) -> S {
    postcard::from_bytes(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

struct Result {
    name: &'static str,
    top_test_name: &'static str,
    size: usize,
    time_se_de: Duration,
    time_se_ms: String,
    time_de_ms: String,
    time_se_de_ms: String,
}

fn se_bench<T, FS: Fn(&T, &mut TestData) + Clone>(
    x: &T,
    serialize: FS,
    data: &mut TestData,
) -> Duration {
    let start = Instant::now();
    for _ in 0..data.iterations {
        data.vec.clear();
        data.storage.clear();
        data.rykv_buffer.clear();
        data.rykv_scratch.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
    }
    start.elapsed()
}

fn de_bench<T, FD: Fn(&TestData) -> T>(deserialize: FD, data: &TestData) -> Duration {
    let start = Instant::now();
    for _ in 0..data.iterations {
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
    for _ in 0..data.iterations {
        data.vec.clear();
        data.storage.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
        black_box(deserialize(black_box(data)));
    }
    start.elapsed()
}

fn bench<T, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    top_test_name: &'static str,
    test_name: &'static str,
    x: &T,
    serialize: FS,
    deserialize: FD,
    results: &mut Vec<Result>,
    iterations: u32,
) {
    let mut data = TestData {
        vec: Vec::default(),
        storage: Storage::default(),
        rykv_buffer: AlignedVec::new(),
        rykv_scratch: AlignedVec::new(),
        iterations,
    };
    let time_se = se_bench(x, serialize.clone(), &mut data);
    let time_de = de_bench(deserialize.clone(), &data);
    let time_se_de = se_de_bench(x, serialize, deserialize, &mut data);

    results.push(Result {
        name: test_name,
        top_test_name,
        size: data.vec.len().max(data.storage.len()),
        time_se_de,
        time_se_ms: format!("{:.2}", time_se.as_secs_f64() * 1000.0),
        time_de_ms: format!("{:.2}", time_de.as_secs_f64() * 1000.0),
        time_se_de_ms: format!("{:.2}", time_se_de.as_secs_f64() * 1000.0),
    });
}

fn add_benches<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    top_test_name: &'static str,
    t: &T,
    s: &S,
    results: &mut Vec<Result>,
    iterations: u32,
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

        fn serialize_to<V: VecLike>(
            &self,
            output: &mut V,
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
    let wrapper = Wrapper(t.clone());

    bench(
        top_test_name,
        "flat_message",
        t,
        se_test_flat_message,
        de_test_flat_message,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "flat_message_unchecked",
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "rmp_schema",
        s,
        se_test_rmp_schema,
        de_test_rmp,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "rmp_schemaless",
        s,
        se_test_rmp_schemaless,
        de_test_rmp,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "bincode",
        s,
        se_test_bincode,
        de_test_bincode,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "flexbuffers",
        s,
        se_test_flexbuffers,
        de_test_flexbuffers,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "cbor",
        s,
        se_test_cbor,
        de_test_cbor,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "bson",
        s,
        se_test_bson,
        de_test_bson,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "json",
        s,
        se_test_json,
        de_test_json,
        results,
        iterations,
    );
    bench(
        top_test_name,
        "postcard",
        s,
        se_test_postcard,
        de_test_postcard,
        results,
        iterations,
    );
}

fn print_results(results: &mut Vec<Result>) {
    results.sort_by(|x, y| {
        x.top_test_name
            .cmp(&y.top_test_name)
            .then(x.time_se_de.cmp(&y.time_se_de))
    });

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);
    ascii_table
        .column(0)
        .set_header("top name")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("name")
        .set_align(Align::Left);
    ascii_table
        .column(2)
        .set_header("size (b)")
        .set_align(Align::Right);
    ascii_table
        .column(3)
        .set_header("se time (ms)")
        .set_align(Align::Right);
    ascii_table
        .column(4)
        .set_header("de time (ms)")
        .set_align(Align::Right);
    ascii_table
        .column(5)
        .set_header("se + de time (ms)")
        .set_align(Align::Right);

    let mut r: Vec<[&dyn Display; 6]> = Vec::new();
    let mut last = None;
    for i in results {
        let current = Some(i.top_test_name);
        if !last.is_none() && last != current {
            r.push([&"---", &"---", &"---", &"---", &"---", &"---"]);
        }
        last = current;

        r.push([
            &i.top_test_name,
            &i.name,
            &i.size,
            &i.time_se_ms,
            &i.time_de_ms,
            &i.time_se_de_ms,
        ]);
    }

    ascii_table.print(r);
}

fn do_one<'a, T: FlatMessageOwned + Clone, S: Serialize + DeserializeOwned>(
    top_test_name: &'static str,
    process: &T,
    process_s: &S,
    results: &mut Vec<Result>,
    iterations: u32,
) {
    add_benches(top_test_name, process, process_s, results, iterations);
}

#[derive(clap::Parser)]
struct Args {
    #[arg(long, short, default_value_t = 1_000_000)]
    iterations: u32,
}

fn main() {
    let args = Args::parse();
    println!("iterations: {}", args.iterations);
    let results = &mut Vec::new();
    {
        let process_small = structures::process_create::generate_flat();
        let process_s_small = structures::process_create::generate_other();
        do_one(
            "process_create",
            &process_small,
            &process_s_small,
            results,
            args.iterations,
        );
    }
    {
        let s = structures::long_strings::generate(100);
        do_one("long_strings", &s, &s, results, args.iterations);
    }
    {
        let s = structures::point::generate();
        do_one("point", &s, &s, results, args.iterations);
    }
    {
        let s = structures::one_bool::generate();
        do_one("one_bool", &s, &s, results, args.iterations);
    }
    {
        let s = structures::multiple_fields::generate();
        do_one("multiple_fields", &s, &s, results, args.iterations);
    }
    {
        let s = structures::multiple_integers::generate();
        do_one("multiple_integers", &s, &s, results, args.iterations);
    }
    {
        let s = structures::vectors::generate();
        do_one("vectors", &s, &s, results, args.iterations);
    }
    {
        let s = structures::large_vectors::generate();
        do_one("large_vectors", &s, &s, results, args.iterations);
    }
    print_results(results);
}
