use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::sort::par_sample_sort;
use crate::sort::seq_sample_sort;
use crate::util::data_generator::*;

fn benchmark_sample_sort(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -10, 11);
        let t = Instant::now();
        seq_sample_sort(&mut arr, 5, 3);
        tot_time += t.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_par_sample_sort(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -10, 11);
        let t = Instant::now();
        par_sample_sort(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
        tot_time += t.elapsed();
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
pub fn run_sample_sort_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();
    println!("Seq");
    let key = format!("{}, {}, sample_sort seq", &d, threads);
    let d1 = benchmark_sample_sort(size, rounds);
    println!("{}: {:?}", key, d1);
    result.entry(key).or_insert(d1);

    println!("Par");
    let key = format!("{}, {}, sample_sort par", &d, threads);
    let d2 = benchmark_par_sample_sort( size,  rounds);
    println!("{}: {:?}", key, d2);
    result.entry(key).or_insert(d2);

    result
}
