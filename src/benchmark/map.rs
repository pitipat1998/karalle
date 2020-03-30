use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::*;
use crate::util::data_generator::*;

// type MapFunc = (dyn Sync + Send + (Fn(usize, &u16) -> u16));

#[allow(dead_code)]
fn fac(i: &u16) -> u16 {
    if (*i) <= 1 {
        1
    } else {
        let f = &fac(&(*i - 1));
        (*i) * (*f)
    }
}

#[allow(dead_code)]
fn benchmark_map_v1(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = par_map_v1(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_map_v3(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = par_map_v3(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_map_v5(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = par_map_v5(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}


#[allow(dead_code)]
pub fn run_map_benchmark(d: &String, size: u64, rounds: u128, threads: usize) -> HashMap<String, Duration> {
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, sqrt_n_{}", &d, threads, "Multiply");
    println!("map v1: {} {}", "Multiply", &d);
    let duration = benchmark_map_v1(size, rounds);
    result.entry(key).or_insert(duration);

    println!("map v3: {} {}", "Multiply", &d);
    let key = format!("{}, {}, half_split_{}", &d, threads, "Multiply");
    let duration = benchmark_map_v3(size, rounds);
    result.entry(key).or_insert(duration);

    println!("rayon map: {} {}", "Multiply", &d);
    let key = format!("{}, {}, rayon_par_iter_{}", &d, threads, "Multiply");
    let duration = benchmark_map_v5(size, rounds);
    result.entry(key).or_insert(duration);

    result
}
