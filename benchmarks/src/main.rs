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
    name: AlgoKind,
    top_test_name: TestKind,
    size: usize,
    needs_schema: bool,
    //
    time_se: Duration,
    time_de: Duration,
    time_se_de: Duration,
    //
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

fn fmt_time_ms(x: Duration) -> String {
    format!("{:.2}", x.as_secs_f64() * 1000.0)
}

fn bench<T, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    top_test_name: TestKind,
    test_name: AlgoKind,
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
        time_se,
        time_de,
        time_se_de,
        time_se_ms: fmt_time_ms(time_se),
        time_de_ms: fmt_time_ms(time_de),
        time_se_de_ms: fmt_time_ms(time_se_de),
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
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    iterations: u32,
) {
    let wrapper = Wrapper(x.clone());

    macro_rules! b {
        ($name:expr, $x:expr, $se:expr, $de:expr, $needs_schema:expr) => {
            if all_algos || algos.contains(&$name) {
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

    use AlgoKind::*;
    b!(
        FlatMessage,
        x,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(
        FlatMessageUnchecked,
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(RmpSchema, x, se_test_rmp_schema, de_test_rmp, true);
    b!(RmpSchemaless, x, se_test_rmp_schemaless, de_test_rmp, false);
    b!(Bincode, x, se_test_bincode, de_test_bincode, true);
    b!(
        FlexBuffers,
        x,
        se_test_flexbuffers,
        de_test_flexbuffers,
        false
    );
    b!(Cbor, x, se_test_cbor, de_test_cbor, false);
    b!(Bson, x, se_test_bson, de_test_bson, false);
    b!(Json, x, se_test_json, de_test_json, false);
    b!(SimdJson, x, se_test_simd_json, de_test_simd_json, false);
    b!(Postcard, x, se_test_postcard, de_test_postcard, true);
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

    let dashes: [&dyn Display; 7] = [&"---", &"---", &"---", &"---", &"---", &"---", &"---"];

    for i in results.iter() {
        let current = Some(i.top_test_name);
        if !last.is_none() && last != current {
            r.push(dashes);
        }
        last = current;

        let ch = if i.needs_schema { &'*' } else { &' ' };
        r.push([
            i.top_test_name.display(),
            ch,
            i.name.display(),
            &i.size,
            &i.time_se_ms,
            &i.time_de_ms,
            &i.time_se_de_ms,
        ]);
    }

    let avg_size = results.iter().map(|x| x.size).sum::<usize>() / results.len();
    let avg_se_time = results.iter().map(|x| x.time_se).sum::<Duration>() / results.len() as u32;
    let avg_de_time = results.iter().map(|x| x.time_de).sum::<Duration>() / results.len() as u32;
    let avg_se_de_time =
        results.iter().map(|x| x.time_se_de).sum::<Duration>() / results.len() as u32;

    let avg_se_time = fmt_time_ms(avg_se_time);
    let avg_de_time = fmt_time_ms(avg_de_time);
    let avg_se_de_time = fmt_time_ms(avg_se_de_time);

    r.push(dashes);
    r.push([
        &"average",
        &"",
        &"",
        &avg_size,
        &avg_se_time,
        &avg_de_time,
        &avg_se_de_time,
    ]);

    print_results_ascii_table(&r, &colums);
    print_results_markdown(&r, &colums);
}

fn do_one<'a, T: FlatMessageOwned + Clone + Serialize + DeserializeOwned>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    iterations: u32,
) {
    add_benches(top_test_name, x, results, algos, all_algos, iterations);
}

macro_rules! tests {
    ($enum_name:ident, $(($name:literal, $v:ident)),+) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
        enum $enum_name {
            $(
                $v,
            )+
        }
        impl $enum_name {
            // fn name(self) -> &'static str {
            //     match self {
            //         $(
            //             Self::$v => $name,
            //         )+
            //     }
            // }
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
                    _ => panic!("unknown option: {}\navailable option: {}", value, Self::all().join(", ")),
                }
            }
        }
    };
}

tests! {
    TestKind,
    ("process_create", ProcessCreate),
    ("long_strings", LongStrings),
    ("point", Point),
    ("multiple_fields", MultipleFields),
    ("multiple_integers", MultipleIntegers),
    ("multiple_bools", MultipleBools),
    ("vectors", Vectors),
    ("large_vectors", LargeVectors),
    ("enum_fields", EnumFields),
    ("enum_lists", EnumLists),
    ("small_enum_lists", SmallEnumLists),
    ("strings_lists", StringLists),
    ("one_bool", OneBool)
}

tests! {
    AlgoKind,
    ("flat_message", FlatMessage),
    ("flat_message_unchecked", FlatMessageUnchecked),
    ("rmp_schema", RmpSchema),
    ("rmp_schemaless", RmpSchemaless),
    ("bincode", Bincode),
    ("flexbuffers" , FlexBuffers),
    ("cbor", Cbor),
    ("bson", Bson),
    ("json", Json),
    ("simd_json", SimdJson),
    ("postcard", Postcard)
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
    let algos_names = AlgoKind::all().join(", ");
    if args.names {
        println!("available tests: {}", test_names);
        println!("available algos: {}", algos_names);
        return;
    }

    let (all_tests, tests) = split_tests::<TestKind>(&args.tests);
    let (all_algos, algos) = split_tests(&args.algos);

    println!("iterations: {}", args.iterations);

    let results = &mut Vec::new();
    macro_rules! run {
        ($name:expr, $x:expr) => {
            if all_tests || tests.contains(&$name) {
                do_one($name, $x, results, &algos, all_algos, args.iterations);
            }
        };
    }

    use TestKind::*;
    {
        let process_small = structures::process_create::generate_flat();
        run!(ProcessCreate, &process_small);
    }
    {
        let s = structures::long_strings::generate(100);
        run!(LongStrings, &s);
    }
    {
        let s = structures::point::generate();
        run!(Point, &s);
    }
    {
        let s = structures::one_bool::generate();
        run!(OneBool, &s);
    }
    {
        let s = structures::multiple_fields::generate();
        run!(MultipleFields, &s);
    }
    {
        let s = structures::multiple_integers::generate();
        run!(MultipleIntegers, &s);
    }
    {
        let s = structures::vectors::generate();
        run!(Vectors, &s);
    }
    {
        let s = structures::large_vectors::generate();
        run!(LargeVectors, &s);
    }
    {
        let s = structures::enum_fields::generate();
        run!(EnumFields, &s);
    }
    {
        let s = structures::enum_lists::generate();
        run!(EnumLists, &s);
    }
    {
        let s = structures::small_enum_lists::generate();
        run!(SmallEnumLists, &s);
    }
    {
        let s = structures::multiple_bools::generate();
        run!(MultipleBools, &s);
    }
    {
        let s = structures::string_lists::generate();
        run!(StringLists, &s);
    }

    print_results(results);
}
