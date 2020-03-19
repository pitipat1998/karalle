use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::sort::par_sample_sort;
use crate::sort::seq_sample_sort;
use num::PrimInt;
use serde::export::fmt::{Display, Debug};

fn benchmark_sample_sort<T>(
    v: &mut Vec<T>,
    rounds: u128,
) -> Duration
where T: Copy + Sync + Send + PrimInt
{
    let now = Instant::now();
    for _ in 0..rounds {
        seq_sample_sort(v.as_mut_slice(), 5, 3);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
fn benchmark_par_sample_sort<T, U>(vec: &mut Vec<T>, func: &U, rounds: u128) -> Duration
    where T: Copy + Sync + Send + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let now = Instant::now();
    for _ in 0..rounds {
        let _ = par_sample_sort(vec, func);
    }
    now.elapsed().div_f32(rounds as f32)
}

#[allow(dead_code)]
pub fn run_sample_sort_benchmark<T, U>(
    d: &String,
    v: &mut Vec<T>,
    f: U,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
    where T: Copy + Sync + Send + PrimInt + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let mut result: HashMap<String, Duration> = HashMap::new();
    let key = format!("{}, {}, sample_sort seq", &d, threads);
    let d1 = benchmark_sample_sort(v, rounds);
    result.entry(key).or_insert(d1);

    let key = format!("{}, {}, sample_sort par", &d, threads);
    let d2 = benchmark_par_sample_sort(v, &f, rounds);
    result.entry(key).or_insert(d2);

    result
}