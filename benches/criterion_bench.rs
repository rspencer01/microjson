use std::path::PathBuf;
use std::fs::read_to_string;
use criterion::{criterion_group, criterion_main, Criterion};

use tinyjson::*;

pub fn massive_random(c: &mut Criterion) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test/massive_random.json");
    let json_payload = read_to_string(&path).unwrap();

    c.bench_function("load", |b| {
        b.iter(|| JSONValue::parse(&json_payload))
    });

    let value = JSONValue::parse(&json_payload);
    c.bench_function("single_retrieve", |b| {
        b.iter(|| {
            assert_eq!(
                value
                    .unwrap()
                    .0
                    .get_nth_array_item(5)
                    .unwrap()
                    .get_key_value("actor")
                    .unwrap()
                    .get_key_value("id")
                    .unwrap()
                    .read_integer(),
                Ok(4319954)
            )
        })
    });
}

pub fn large_array(c: &mut Criterion) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test/list_of_squares.json");
    let json_payload = read_to_string(&path).unwrap();

    c.bench_function("load_array", |b| {
        b.iter(|| JSONValue::parse(&json_payload))
    });

    let json = JSONValue::parse(&json_payload).unwrap().0;
    c.bench_function("read_array_sequentially", |b| {
        b.iter(|| {
            for i in 0..10007 {
                assert_eq!(json.get_nth_array_item(i as usize).unwrap().read_integer(), Ok(i * i));
            }
        })
    });
    c.bench_function("read_array_out_of_order", |b| {
        b.iter(|| {
            let mut i = 1;
            for _ in 0..10007 {
                assert_eq!(json.get_nth_array_item(i as usize).unwrap().read_integer(), Ok(i * i));
                i = (i * 5) % 10007;
            }
        })
    });
}

criterion_group!(benches, massive_random, large_array);
criterion_main!(benches);
