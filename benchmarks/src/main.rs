use ascii_table::{Align, AsciiTable};
use flat_message::{Storage, FlatMessage, FlatMessageOwned, VecLike};
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

// ----------------------------------------------------------------------------

fn se_test_flat_message<'a, T: FlatMessage<'a>>(
    process: &T,
    output: &mut Vec<u8>,
    _: &mut Storage,
) {
    process
        .serialize_to(output, flat_message::Config::default())
        .unwrap();
}

fn de_test_flat_message<T: FlatMessageOwned>(_: &[u8], input_aligned: &Storage) -> T {
    T::deserialize_from(input_aligned).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    *output = bson::to_vec(&process).unwrap();
}

fn de_test_bson<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    bson::from_slice(&input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_cbor<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    ciborium::into_writer(process, &mut *output).unwrap();
}

fn de_test_cbor<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    ciborium::from_reader(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_json<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    serde_json::to_writer(&mut *output, process).unwrap();
}

fn de_test_json<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    serde_json::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_rmp_schema<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    rmp_serde::encode::write(output, process).unwrap();
}

fn se_test_rmp_schemaless<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    rmp_serde::encode::write_named(output, process).unwrap();
}

fn de_test_rmp<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    rmp_serde::decode::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bincode<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    bincode::serialize_into(&mut *output, process).unwrap();
}

fn de_test_bincode<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    bincode::deserialize(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_flexbuffers<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    *output = flexbuffers::to_vec(process).unwrap();
}

fn de_test_flexbuffers<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    flexbuffers::from_slice(input).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_postcard<S: Serialize>(process: &S, output: &mut Vec<u8>, _: &mut Storage) {
    postcard::to_io(process, output).unwrap();
}

fn de_test_postcard<S: DeserializeOwned>(input: &[u8], _: &Storage) -> S {
    postcard::from_bytes(input).unwrap()
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

const ITERATIONS: u32 = 1_000_000;

fn se_bench<T, FS: Fn(&T, &mut Vec<u8>, &mut Storage) + Clone>(
    x: &T,
    serialize: FS,
    vec: &mut Vec<u8>,
    aligned_vec: &mut Storage,
) -> Duration {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        vec.clear();
        black_box(serialize(x, vec, aligned_vec));
        black_box(vec.len());
    }
    start.elapsed()
}

fn de_bench<T, FD: Fn(&[u8], &Storage) -> T>(
    deserialize: FD,
    input: &[u8],
    input_aligned: &Storage,
) -> Duration {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(deserialize(black_box(input), black_box(input_aligned)));
    }
    start.elapsed()
}

fn se_de_bench<
    T,
    FS: Fn(&T, &mut Vec<u8>, &mut Storage) + Clone,
    FD: Fn(&[u8], &Storage) -> T + Clone,
>(
    x: &T,
    serialize: FS,
    deserialize: FD,
    vec: &mut Vec<u8>,
    aligned_vec: &mut Storage,
) -> Duration {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        vec.clear();
        black_box(serialize(x, vec, aligned_vec));
        black_box(vec.len());
        black_box(deserialize(black_box(vec), black_box(aligned_vec)));
    }
    start.elapsed()
}

fn bench<
    T,
    FS: Fn(&T, &mut Vec<u8>, &mut Storage) + Clone,
    FD: Fn(&[u8], &Storage) -> T + Clone,
>(
    top_test_name: &'static str,
    test_name: &'static str,
    x: &T,
    serialize: FS,
    deserialize: FD,
    results: &mut Vec<Result>,
) {
    let vec = &mut Vec::default();
    let aligned_vec = &mut Storage::default();
    let time_se = se_bench(x, serialize.clone(), vec, aligned_vec);
    let time_de = de_bench(deserialize.clone(), vec, aligned_vec);
    let time_se_de = se_de_bench(x, serialize, deserialize, vec, aligned_vec);

    results.push(Result {
        name: test_name,
        top_test_name,
        size: vec.len(),
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
    bench(
        top_test_name,
        "postcard",
        s,
        se_test_postcard,
        de_test_postcard,
        results,
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
) {
    add_benches(top_test_name, process, process_s, results);
}

fn main() {
    println!("iterations: {}", ITERATIONS);
    let results = &mut Vec::new();
    {
        let process_small = structures::process_create::generate_flat();
        let process_s_small = structures::process_create::generate_other();
        do_one("process_create", &process_small, &process_s_small, results);
    }
    {
        let s = structures::long_strings::generate(100);
        do_one("long_strings", &s, &s, results);
    }
    {
        let s = structures::point::generate();
        do_one("point", &s, &s, results);
    }
    {
        let s = structures::one_bool::generate();
        do_one("one_bool", &s, &s, results);
    }
    {
        let s = structures::multiple_fields::generate();
        do_one("multiple_fields", &s, &s, results);
    }
    {
        let s = structures::multiple_integers::generate();
        do_one("multiple_integers", &s, &s, results);
    }
    {
        let s = structures::vectors::generate();
        do_one("vectors", &s, &s, results);
    }
    print_results(results);
}
