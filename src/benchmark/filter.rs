use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::{par_filter_v2, par_filter_v3};
use crate::util::random_i16_list_generator;

#[allow(dead_code)]
fn run_par_filter(size: u64, rounds: u128) -> Duration {
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let vec: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _r: Vec<i16> = par_filter_v2(&vec, &|_i: usize, a: &i16| -> bool { *a < 3 });
        tot_time += now.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn run_rayon_filter(size: u64, rounds: u128) -> Duration {
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let vec: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _r: Vec<i16> = par_filter_v3(&vec, &|_i: usize, a: &i16| -> bool { *a < 3 });
        tot_time += now.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
pub fn run_filter_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_filter", &d, threads);
    let duration = run_par_filter(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, rayon_filter", &d, threads);
    let duration = run_rayon_filter(size, rounds);
    result.entry(key).or_insert(duration);

    result
}