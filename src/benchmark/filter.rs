use std::collections::HashMap;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::primitive::{par_filter_v2, par_filter_v3};

fn run_par_filter(vec: &mut Vec<i32>, rounds: u128) -> Duration {
    let now = Instant::now();
    for _ in 0..rounds {
        let _r: Vec<i32> = par_filter_v2(&vec, &|_i: usize, a: &i32| -> bool { *a < 3 });
    }
    now.elapsed().div_f32(rounds as f32)
}

fn run_rayon_filter(vec: &mut Vec<i32>, rounds: u128) -> Duration {
    let now = Instant::now();
    for _ in 0..rounds {
        let _r: Vec<i32> = par_filter_v3(&vec, &|_i: usize, a: &i32| -> bool { *a < 3 });
    }
    now.elapsed().div_f32(rounds as f32)
}


pub fn run_filter_benchmark(
    d: &String,
    v: &mut Vec<i32>,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_filter", &d, threads);
    let duration = run_par_filter(v, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, rayon_filter", &d, threads);
    let duration = run_rayon_filter(v, rounds);
    result.entry(key).or_insert(duration);

    result
}