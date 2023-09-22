use criterion::{black_box, criterion_group, criterion_main, Criterion};
use utils::frequency;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file(file_path: &str) -> Vec<u16> {
    let file = File::open(file_path).expect("file wasn't found");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    let numbers_100: &[u16] = &load_from_file("randoms/100randoms.txt");
    let numbers_1000: &[u16] = &load_from_file("randoms/1000randoms.txt");
    let numbers_10000: &[u16] = &load_from_file("randoms/10000randoms.txt");
    let numbers_100000: &[u16] = &load_from_file("randoms/100000randoms.txt");
    let numbers_1000000: &[u16] = &load_from_file("randoms/1000000randoms.txt");

    c.bench_function("freq 100", |b| b.iter(|| frequency(black_box(numbers_100))));
    c.bench_function("freq 1000", |b| {
        b.iter(|| frequency(black_box(numbers_1000)))
    });
    c.bench_function("freq 10000", |b| {
        b.iter(|| frequency(black_box(numbers_10000)))
    });
    c.bench_function("freq 100000", |b| {
        b.iter(|| frequency(black_box(numbers_100000)))
    });
    c.bench_function("freq 1000000", |b| {
        b.iter(|| frequency(black_box(numbers_1000000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
