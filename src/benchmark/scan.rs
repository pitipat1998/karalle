use std::collections::HashMap;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::primitive::*;

#[allow(dead_code)]
fn benchmark_scan_rayon(vec: &Vec<i32>, rounds: u128) -> Duration
{
    let now = Instant::now();
    for _ in 0..rounds {
        let _: i32 = vec.into_par_iter().sum();
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
fn benchmark_scan(vec: &mut Vec<i32>, rounds: u128) -> Duration
{
    let now = Instant::now();
    for _ in 0..rounds {
        let _ = par_scan(vec, |&a, &b| { a + b }, &0);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
pub fn run_scan_benchmark<U>(
    d: &String,
    v: &mut Vec<i32>,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
    where U: Sync + Send + Fn(&i32, &i32) -> i32
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_scan", &d, threads);
    let duration = benchmark_scan(v, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, rayon_sum", &d, threads);
    let duration = benchmark_scan_rayon(v, rounds);
    result.entry(key).or_insert(duration);

    result
}