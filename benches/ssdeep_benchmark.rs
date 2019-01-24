#[macro_use]
extern crate criterion;

use criterion::Criterion;
use fuzzy::ssdeep::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{remove_file, File};
use std::io::prelude::*;

fn random_str(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}

fn hash_100k(c: &mut Criterion) {
    let sample = random_str(100000);
    c.bench_function("hash rand 100K", move |b| b.iter(|| Hash::hash(&sample)));
}

fn hash_1m(c: &mut Criterion) {
    let sample = random_str(1000000);
    c.bench_function("hash rand 1M", move |b| b.iter(|| Hash::hash(&sample)));
}

// fn hash_10m(c: &mut Criterion) {
//     let sample = random_str(10000000);
//     c.bench_function("hash rand 10M", move |b| b.iter(|| Hash::hash(&sample)));
// }

fn hash_from_file(c: &mut Criterion) {
    let sample = random_str(100000);
    let mut file = File::create("a.txt").unwrap();
    let _ = file.write_all(sample.as_bytes()).unwrap();
    c.bench_function("hash from file rand 100K", move |b| {
        b.iter(|| Hash::hash_from_file("a.txt"))
    });
    let _ = remove_file("a.txt").unwrap();
}

fn compare(c: &mut Criterion) {
    let sample_a = random_str(100000);
    let sample_b = random_str(100000);
    let digest_a = Hash::hash(&sample_a).unwrap();
    let digest_b = Hash::hash(&sample_b).unwrap();
    c.bench_function("compare rand 100K", move |b| {
        b.iter(|| Hash::compare(&digest_a, &digest_b))
    });
}

criterion_group!(
    benches,
    hash_100k,
    hash_1m,
    // hash_10m,
    compare,
    hash_from_file
);
criterion_main!(benches);
