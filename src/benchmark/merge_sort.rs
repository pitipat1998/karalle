extern crate rayon;

use std::time::{Duration, Instant};
use std::collections::HashMap;
use rayon::prelude::*;
use crate::util::random_i16_list_generator;

#[allow(dead_code)]
pub fn run_merge_sort_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize) -> HashMap<String, Duration> {
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_mergesort", &d, threads);
    let mut d = Duration::new(0,0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let now = Instant::now();
        arr.as_parallel_slice_mut().par_sort();
        d += now.elapsed();
    }
    result.entry(key).or_insert(d.div_f32(rounds as f32));
    result
}