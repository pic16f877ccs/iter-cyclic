#![feature(test)]
extern crate test;

use iter_cyclic::range_skip;
use test::Bencher;

#[bench]
fn bench_vec_create(bench: &mut Bencher) {
    bench.iter(|| {
        let vec: Vec<u32> = vec![1; 10_000_000];
    });
}

#[bench]
fn bench_range_skip(bench: &mut Bencher) {
    bench.iter(|| {
        let vec: Vec<u32> = range_skip(0_u32..1, 1)
            .take(10_000_000).collect();
    });
}

