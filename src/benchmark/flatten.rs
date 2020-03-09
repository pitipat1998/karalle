use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::*;

// par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
#[allow(dead_code)]
type FlattenFunc<T> = dyn Sync + Send + Fn(&Vec<&Vec<T>>) -> Vec<T>;

#[allow(dead_code)]
fn benchmark_flatten<T: Copy + Sync, K>(vec: &Vec<&Vec<T>>, flat: K, rounds: u128) -> Duration
    where K: Sync + Send + (Fn(&Vec<&Vec<T>>) -> Vec<T>)
{
    let now = Instant::now();
    for _ in 0..rounds {
        let _ = flat(&vec);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
pub fn run_flatten_benchmark<T>(
    d: &String,
    v: &Vec<&Vec<T>>,
    rounds: u128,
    threads: usize
) -> HashMap<String, Duration>
where T: Copy + Sync + Send
{
    let mut result: HashMap<String, Duration> = HashMap::new();
    let key = format!("{}, {},flatten", &d, threads);
    let duration = benchmark_flatten(&v, par_flatten, rounds);
    result.entry(key).or_insert(duration);
    result
}