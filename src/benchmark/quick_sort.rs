use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::sort::*;
use crate::util::data_generator::*;
use super::sample_sort::benchmark_par_sample_sort;

#[allow(dead_code)]
fn benchmark_non_inplace_par_quicksort(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        non_inplace_par_quicksort(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
        tot_time += t.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_par_quicksort(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        par_quicksort(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
        tot_time += t.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_rayon_par_quicksort(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        rayon_par_quicksort(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
        tot_time += t.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
pub fn run_quick_sort_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_quick_sort (non-in-place)", &d, threads);
    let duration = benchmark_non_inplace_par_quicksort(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, par_quick_sort (in-place)", &d, threads);
    let duration = benchmark_par_quicksort(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, par_quick_sort (rayon)", &d, threads);
    let duration = benchmark_rayon_par_quicksort(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, par_sample_sort (in-place)", &d, threads);
    let duration = benchmark_par_sample_sort(size, rounds);
    result.entry(key).or_insert(duration);
    result
}