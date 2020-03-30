use std::cmp::min;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::Rng;

use crate::primitive::*;
use crate::util::data_generator::*;

#[allow(dead_code)]
type FlattenFunc<T> = dyn Sync + Send + Fn(&Vec<&Vec<T>>) -> Vec<T>;

#[allow(dead_code)]
fn benchmark_flatten_v1(size: u64, rounds: u128) -> Duration
{
    let mut rng = rand::thread_rng();
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut c_size = size;
        let mut arr: Vec<Vec<i16>> = Vec::with_capacity(size as usize);
        while c_size > 0 {
            let s: u64 = if c_size == 1 { 1 } else {
                rng.gen_range(1, min(c_size, 1000001))
            };
            let tmp = random_i16_list_generator(s, -1000, 1001);
            arr.push(tmp);
            c_size -= s;
        }
        let now = Instant::now();
        let _ = par_flatten(&arr);
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_flatten_v2(size: u64, rounds: u128) -> Duration
{
    let mut rng = rand::thread_rng();
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut c_size = size;
        let mut arr: Vec<Vec<i16>> = Vec::with_capacity(size as usize);
        while c_size > 0 {
            let s: u64 = if c_size == 1 { 1 } else {
                rng.gen_range(1, min(c_size, 1000001))
            };
            let tmp = random_i16_list_generator(s, -1000, 1001);
            arr.push(tmp);
            c_size -= s;
        }
        let now = Instant::now();
        let _ = par_flatten_v2(&arr);
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
pub fn run_flatten_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize,
) -> HashMap<String, Duration>
{
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, par_flatten", &d, threads);
    let duration = benchmark_flatten_v1(size, rounds);
    result.entry(key).or_insert(duration);

    let key = format!("{}, {}, rayon_par_iter", &d, threads);
    let duration = benchmark_flatten_v2(size, rounds);
    result.entry(key).or_insert(duration);
    result
}