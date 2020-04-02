use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::*;
use crate::util::random_i16_list_generator;

#[allow(dead_code)]
fn benchmark_scan_rayon(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _:Vec<i16> = arr.iter().scan(0, |state, &x|{
            *state = *state+x;
            Some(*state)
        }).collect();
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_scan(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = par_scan(arr.as_mut_slice(), |a: &i16, b: &i16| -> i16 { *a + *b },&0,);
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
pub fn run_scan_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_scan", &d, threads);
    let duration = benchmark_scan(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, rayon_sum", &d, threads);
    let duration = benchmark_scan_rayon(size, rounds);
    result.entry(key).or_insert(duration);

    result
}