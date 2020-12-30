use criterion::{criterion_group, criterion_main, Criterion};

extern crate chordclust;

use chordclust::{cluster_hierarchical, read_fasta_sorted};
use std::fs::File;

fn solve_hierarchical() {
    let f = File::open("examples/UP000000425_122586_DNA_sample.fasta").unwrap();
    let sequences = read_fasta_sorted(f);
    let thresholds = [90, 70, 50];
    cluster_hierarchical(&sequences, 8, &thresholds);
}

fn _solve_hierarchical_big() {
    let f = File::open("examples/UP000000425_122586_DNA.fasta").unwrap();
    let sequences = read_fasta_sorted(f);
    let thresholds = [90, 70, 50];
    cluster_hierarchical(&sequences, 8, &thresholds);
}

fn hierarchical_benchmark(c: &mut Criterion) {
    c.bench_function("Run hierararchical with sample file", |b| b.iter(solve_hierarchical));
}

fn _hierarchical_big_benchmark(c: &mut Criterion) {
    c.bench_function("Run hierararchical with big file", |b| b.iter(_solve_hierarchical_big));
}

criterion_group!(
    benches,
    hierarchical_benchmark,
);
criterion_main!(benches);
