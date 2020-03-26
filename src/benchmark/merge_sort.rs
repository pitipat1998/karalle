extern crate rayon;

use std::time::{Duration, Instant};
use std::collections::HashMap;
use rayon::prelude::*;

#[allow(dead_code)]
pub fn run_merge_sort_benchmark(
    d: &String,
    v: &mut Vec<i32>,
    rounds: u128,
    threads: usize) -> HashMap<String, Duration> {
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_mergesort", &d, threads);
    let now = Instant::now();
    for _ in 0..rounds {
        v.as_parallel_slice_mut().par_sort()
    }
    result.entry(key).or_insert(now.elapsed().div_f32(rounds as f32));
    result
}