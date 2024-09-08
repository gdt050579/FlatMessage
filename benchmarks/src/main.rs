use ascii_table::{Align, AsciiTable};
use clap::Parser;
use flat_message::{FlatMessage, FlatMessageOwned, Storage, VecLike};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Write;
use std::fs;
use std::hash::Hash;
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

fn se_test_simd_json<S: Serialize>(process: &S, data: &mut TestData) {
    simd_json::serde::to_writer(&mut data.vec, process).unwrap();
}

fn de_test_simd_json<S: DeserializeOwned>(data: &TestData) -> S {
    simd_json::serde::from_reader(data.vec.as_slice()).unwrap()
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

struct Result {
    name: &'static str,
    top_test_name: TestKind,
    size: usize,
    time_se_de: Duration,
    time_se_ms: String,
    time_de_ms: String,
    time_se_de_ms: String,
    needs_schema: bool,
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
    top_test_name: TestKind,
    test_name: &'static str,
    x: &T,
    serialize: FS,
    deserialize: FD,
    needs_schema: bool,
    results: &mut Vec<Result>,
    iterations: u32,
) {
    let mut data = TestData {
        vec: Vec::default(),
        storage: Storage::default(),
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
        needs_schema,
    });
}

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

fn add_benches<'a, T: FlatMessageOwned + Clone + Serialize + DeserializeOwned>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &mut HashSet<&str>,
    all_algos: bool,
    iterations: u32,
) {
    let wrapper = Wrapper(x.clone());

    macro_rules! b {
        ($name:literal, $x:expr, $se:expr, $de:expr, $needs_schema:expr) => {
            if all_algos || algos.remove($name) {
                bench(
                    top_test_name,
                    $name,
                    $x,
                    $se,
                    $de,
                    $needs_schema,
                    results,
                    iterations,
                );
            }
        };
    }

    b!(
        "flat_message",
        x,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(
        "flat_message_unchecked",
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!("rmp_schema", x, se_test_rmp_schema, de_test_rmp, true);
    b!(
        "rmp_schemaless",
        x,
        se_test_rmp_schemaless,
        de_test_rmp,
        false
    );
    b!("bincode", x, se_test_bincode, de_test_bincode, true);
    b!(
        "flexbuffers",
        x,
        se_test_flexbuffers,
        de_test_flexbuffers,
        false
    );
    b!("cbor", x, se_test_cbor, de_test_cbor, false);
    b!("bson", x, se_test_bson, de_test_bson, false);
    b!("json", x, se_test_json, de_test_json, false);
    b!("simd_json", x, se_test_simd_json, de_test_simd_json, false);
    b!("postcard", x, se_test_postcard, de_test_postcard, true);
}

fn print_results_ascii_table(r: &[[&dyn Display; 7]], colums: &[(&str, Align)]) {
    let mut ascii_table: AsciiTable = AsciiTable::default();
    ascii_table.set_max_width(150);

    for (i, (name, align)) in colums.iter().enumerate() {
        ascii_table.column(i).set_header(*name).set_align(*align);
    }

    ascii_table.print(r);
}

fn print_results_markdown(r: &[[&dyn Display; 7]], colums: &[(&str, Align)]) {
    let output = &mut String::with_capacity(4096);

    for i in colums {
        write!(output, "| {} ", i.0).unwrap();
    }
    writeln!(output, "|").unwrap();
    for _ in colums {
        write!(output, "| --- ").unwrap();
    }
    writeln!(output, "|").unwrap();

    for row in r {
        for i in row {
            write!(output, "| {} ", i).unwrap();
        }
        writeln!(output, "|").unwrap();
    }

    fs::write("bench_table.md", output).unwrap();
}

fn print_results(results: &mut Vec<Result>) {
    results.sort_by(|x, y| {
        x.top_test_name
            .cmp(&y.top_test_name)
            .then(x.time_se_de.cmp(&y.time_se_de))
    });

    let colums = [
        ("top name", Align::Left),
        ("schema", Align::Center),
        ("name", Align::Left),
        ("size (b)", Align::Right),
        ("se time (ms)", Align::Right),
        ("de time (ms)", Align::Right),
        ("se + de time (ms)", Align::Right),
    ];

    let mut r: Vec<[&dyn Display; 7]> = Vec::new();
    let mut last = None;

    for i in results {
        let current = Some(i.top_test_name);
        if !last.is_none() && last != current {
            r.push([&"---", &"---", &"---", &"---", &"---", &"---", &"---"]);
        }
        last = current;

        let ch = if i.needs_schema { &'*' } else { &' ' };
        r.push([
            i.top_test_name.display(),
            ch,
            &i.name,
            &i.size,
            &i.time_se_ms,
            &i.time_de_ms,
            &i.time_se_de_ms,
        ]);
    }

    print_results_ascii_table(&r, &colums);
    print_results_markdown(&r, &colums);
}

fn do_one<'a, T: FlatMessageOwned + Clone + Serialize + DeserializeOwned>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &mut HashSet<&str>,
    all_algos: bool,
    iterations: u32,
) {
    add_benches(top_test_name, x, results, algos, all_algos, iterations);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum TestKind {
    ProcessCreate,
    LongStrings,
    Point,
    OneBool,
    MultipleFields,
    MultipleIntegers,
    Vectors,
    LargeVectors,
    EnumFields,
    EnumLists,
    SmallEnumLists,
    MultipleBools,
    StringLists,
}
macro_rules! tests {
    ($enum_name:ident, $(($name:literal, $v:ident)),+) => {
        impl $enum_name {
            fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$v => $name,
                    )+
                }
            }
            fn display(self) -> &'static dyn Display {
                match self {
                    $(
                        Self::$v => &$name,
                    )+
                }
            }
            fn all() -> &'static [&'static str] {
                &[
                    $(
                        $name,
                    )+
                ]
            }
        }
        impl From<&str> for $enum_name {
            fn from(value: &str) -> Self {
                match value {
                    $(
                        $name => Self::$v,
                    )+
                    _ => panic!("unknown test: {}", value),
                }
            }
        }
    };
}
tests! {
    TestKind,
    ("process_create", ProcessCreate),
    ("long_strings" , LongStrings),
    ("point" , Point),
    ("multiple_fields" , MultipleFields),
    ("multiple_integers" , MultipleIntegers),
    ("multiple_bools" , MultipleBools),
    ("vectors" , Vectors),
    ("large_vectors" , LargeVectors),
    ("enum_fields" , EnumFields),
    ("enum_lists" , EnumLists),
    ("small_enum_lists" , SmallEnumLists),
    ("strings_lists" , StringLists),
    ("one_bool" , OneBool)
}

fn split_tests<'x, T>(input: &'x str) -> (bool, HashSet<T>)
where
    T: From<&'x str> + Eq + Hash,
{
    if input == "all" {
        (true, HashSet::new())
    } else {
        let tests = input.split(',').map(|x| T::from(x)).collect();
        (false, tests)
    }
}

#[derive(clap::Parser)]
struct Args {
    #[arg(long, short, default_value_t = 1_000_000)]
    iterations: u32,
    #[arg(long, short, default_value = "all")]
    tests: String,
    #[arg(long, short, default_value = "all")]
    algos: String,
    #[arg(long, short, default_value_t = false)]
    names: bool,
}

fn main() {
    let args = Args::parse();

    let test_names = TestKind::all().join(", ");
    if args.names {
        println!("available tests: {}", test_names);
        return;
    }

    let (all_tests, mut tests) = split_tests::<TestKind>(&args.tests);
    let (all_algos, mut algos) = split_tests(&args.algos);

    println!("iterations: {}", args.iterations);

    macro_rules! run {
        ($name:expr, $($args:expr),+) => {
            if all_tests || tests.remove(&$name) {
                do_one($name, $($args),+);
            }
        };
    }

    let results = &mut Vec::new();
    {
        let process_small = structures::process_create::generate_flat();
        run!(
            TestKind::ProcessCreate,
            &process_small,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::long_strings::generate(100);
        run!(
            TestKind::LongStrings,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::point::generate();
        run!(
            TestKind::Point,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::one_bool::generate();
        run!(
            TestKind::OneBool,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::multiple_fields::generate();
        run!(
            TestKind::MultipleFields,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::multiple_integers::generate();
        run!(
            TestKind::MultipleIntegers,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::vectors::generate();
        run!(
            TestKind::Vectors,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::large_vectors::generate();
        run!(
            TestKind::LargeVectors,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::enum_fields::generate();
        run!(
            TestKind::EnumFields,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::enum_lists::generate();
        run!(
            TestKind::EnumLists,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::small_enum_lists::generate();
        run!(
            TestKind::SmallEnumLists,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::multiple_bools::generate();
        run!(
            TestKind::MultipleBools,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }
    {
        let s = structures::string_lists::generate();
        run!(
            TestKind::StringLists,
            &s,
            results,
            &mut algos,
            all_algos,
            args.iterations
        );
    }

    print_results(results);
    if !tests.is_empty() {
        eprintln!(
            "error: tests not found: {}\navailable tests: {}",
            tests
                .into_iter()
                .map(|x| x.name())
                .collect::<Vec<_>>()
                .join(", "),
            test_names
        );
        std::process::exit(1);
    }
}
