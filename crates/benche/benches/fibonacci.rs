#![feature(test)]

extern crate test;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;

use benche::fibonacci::{fibonacci, fibonacci_sequence, inline_fibonacci, match_fibonacci};
use test::Bencher;

// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>())
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<u32>>())
}

#[bench]
fn fibonacci_match(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(match_fibonacci).collect::<Vec<u32>>())
}

#[bench]
fn fibonacci_inline(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(inline_fibonacci).collect::<Vec<u32>>())
}
