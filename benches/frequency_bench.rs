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

fn rand_freq(c: &mut Criterion) {
    let rand_numbers_100: &[u16] = &load_from_file("randoms/100randoms.txt");
    let rand_numbers_1000: &[u16] = &load_from_file("randoms/1000randoms.txt");
    let rand_numbers_10000: &[u16] = &load_from_file("randoms/10000randoms.txt");
    let rand_numbers_100000: &[u16] = &load_from_file("randoms/100000randoms.txt");
    let rand_numbers_1000000: &[u16] = &load_from_file("randoms/1000000randoms.txt");

    c.bench_function("freq rand 100", |b| {
        b.iter(|| frequency(black_box(rand_numbers_100)))
    });
    c.bench_function("freq rand 1000", |b| {
        b.iter(|| frequency(black_box(rand_numbers_1000)))
    });
    c.bench_function("freq rand 10000", |b| {
        b.iter(|| frequency(black_box(rand_numbers_10000)))
    });
    c.bench_function("freq rand 100000", |b| {
        b.iter(|| frequency(black_box(rand_numbers_100000)))
    });
    c.bench_function("freq rand 1000000", |b| {
        b.iter(|| frequency(black_box(rand_numbers_1000000)))
    });
}

fn seq_freq(c: &mut Criterion) {
    c.bench_function("freq seq 100", |b| {
        b.iter(|| frequency(black_box(&(0..100).collect::<Vec<u32>>())))
    });
    c.bench_function("freq seq 1000", |b| {
        b.iter(|| frequency(black_box(&(0..1000).collect::<Vec<u32>>())))
    });
    c.bench_function("freq seq 10000", |b| {
        b.iter(|| frequency(black_box(&(0..10000).collect::<Vec<u32>>())))
    });
    c.bench_function("freq seq 100000", |b| {
        b.iter(|| frequency(black_box(&(0..100000).collect::<Vec<u32>>())))
    });
    c.bench_function("freq seq 1000000", |b| {
        b.iter(|| frequency(black_box(&(0..1000000).collect::<Vec<u32>>())))
    });
}

criterion_group!(freq, rand_freq, seq_freq);
criterion_main!(freq);
