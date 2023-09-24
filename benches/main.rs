use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fxhash::FxHashMap;
use hashbrown::HashMap as BrownHashMap;
use rand::prelude::*;
use rustc_hash::FxHashMap as RustcHashMap;

fn hash_map_insert_std(input: &Vec<u64>) {
    let mut map_std = std::collections::HashMap::new();
    for &item in input {
        map_std.insert(item, item);
    }
}

fn hash_map_get_std(input: &Vec<u64>, map: &std::collections::HashMap<u64, u64>) {
    for &item in input {
        black_box(map.get(&item));
    }
}

fn hash_map_insert_brown(input: &Vec<u64>) {
    let mut map_brown: BrownHashMap<u64, u64> = BrownHashMap::default();
    for &item in input {
        map_brown.insert(item, item);
    }
}

fn hash_map_get_brown(input: &Vec<u64>, map: &BrownHashMap<u64, u64>) {
    for &item in input {
        black_box(map.get(&item));
    }
}

fn hash_map_insert_rustc(input: &Vec<u64>) {
    let mut map_fx = RustcHashMap::default();
    for &item in input {
        map_fx.insert(item, item);
    }
}

fn hash_map_get_rustc(input: &Vec<u64>, map: &RustcHashMap<u64, u64>) {
    for &item in input {
        black_box(map.get(&item));
    }
}
fn hash_map_insert_fx(input: &Vec<u64>) {
    let mut map_fx = FxHashMap::default();
    for &item in input {
        map_fx.insert(item, item);
    }
}

fn hash_map_get_fx(input: &Vec<u64>, map: &FxHashMap<u64, u64>) {
    for &item in input {
        black_box(map.get(&item));
    }
}

fn benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: Vec<u64> = (0..1_000_000).map(|_| rng.gen_range(0..100_000)).collect();

    let map_brown = input
        .iter()
        .copied()
        .zip(input.iter().copied())
        .collect::<BrownHashMap<_, _>>();
    let map_rustc = input
        .iter()
        .copied()
        .zip(input.iter().copied())
        .collect::<RustcHashMap<_, _>>();
    let map_fx = input
        .iter()
        .copied()
        .zip(input.iter().copied())
        .collect::<FxHashMap<_, _>>();
    let map_std = input
        .iter()
        .copied()
        .zip(input.iter().copied())
        .collect::<std::collections::HashMap<_, _>>();

    c.bench_function("hashbrown insert", |b| {
        b.iter(|| hash_map_insert_brown(&input))
    });
    c.bench_function("hashbrown get", |b| {
        b.iter(|| hash_map_get_brown(&input, &map_brown))
    });
    c.bench_function("rustc_hash insert", |b| {
        b.iter(|| hash_map_insert_rustc(&input))
    });
    c.bench_function("rustc_hash get", |b| {
        b.iter(|| hash_map_get_rustc(&input, &map_rustc))
    });
    c.bench_function("fxhash insert", |b| b.iter(|| hash_map_insert_fx(&input)));
    c.bench_function("fxhash get", |b| {
        b.iter(|| hash_map_get_fx(&input, &map_fx))
    });
    c.bench_function("std insert", |b| b.iter(|| hash_map_insert_std(&input)));
    c.bench_function("std get", |b| b.iter(|| hash_map_get_std(&input, &map_std)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
