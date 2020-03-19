use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::prelude::SliceRandom;

use crate::sort::*;

#[allow(dead_code)]
fn benchmark_quick_sort<T, U>(vec: &Vec<T>, func: &U, rounds: u128) -> Duration
    where T: Copy + Sync + Send,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let now = Instant::now();
    for _ in 0..rounds {
        let _ = par_quick_sort(vec, func);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
fn benchmark_quick_sort_v2<T, U>(vec: &mut Vec<T>, func: &U, rounds: u128) -> Duration
    where T: Copy + Sync + Send,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let now = Instant::now();
    for _ in 0..rounds {
        par_quick_sort_v2(vec, func);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
fn benchmark_quick_sort_v3<T, U>(vec: &mut Vec<T>, func: &U, rounds: u128) -> Duration
    where T: Copy + Sync + Send,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let now = Instant::now();
    for _ in 0..rounds {
        par_quick_sort_v3(vec, func);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
pub fn run_quick_sort_benchmark<T, U>(
    d: &String,
    v: &mut Vec<T>,
    f: U,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
    where T: Copy + Sync + Send,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let mut rng = rand::thread_rng();
    let key = format!("{}, {}, par_quick_sort (non-in-place)", &d, threads);
    let duration = benchmark_quick_sort(&v, &f, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, par_quick_sort (in-place)", &d, threads);
    let duration = benchmark_quick_sort_v2(v, &f, rounds);
    result.entry(key).or_insert(duration);

    v.shuffle(&mut rng);
    let key = format!("{}, {}, par_quick_sort (rayon)", &d, threads);
    let duration = benchmark_quick_sort_v3(v, &f, rounds);
    result.entry(key).or_insert(duration);
    result
}