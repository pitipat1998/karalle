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
fn benchmark_sqrt_splits_par_map(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = sqrt_splits_par_map(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_par_map(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = par_map(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}

#[allow(dead_code)]
fn benchmark_rayon_par_map(size: u64, rounds: u128) -> Duration
{
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -100, 100);
        let now = Instant::now();
        let _ = rayon_par_map(&arr, &|_, x| { *x * *x });
        tot_time += now.elapsed()
    }
    tot_time.div_f64(rounds as f64)
}


#[allow(dead_code)]
pub fn run_map_benchmark(d: &String, size: u64, rounds: u128, threads: usize) -> HashMap<String, Duration> {
    let mut result: HashMap<String, Duration> = HashMap::new();

    let key = format!("{}, {}, sqrt_n_{}", &d, threads, "Multiply");
    println!("par_map sqrt_n: {} {}", "Multiply", &d);
    let duration = benchmark_sqrt_splits_par_map(size, rounds);
    result.entry(key).or_insert(duration);

    println!("par_map : {} {}", "Multiply", &d);
    let key = format!("{}, {}, half_split_{}", &d, threads, "Multiply");
    let duration = benchmark_par_map(size, rounds);
    result.entry(key).or_insert(duration);

    println!("rayon par_map: {} {}", "Multiply", &d);
    let key = format!("{}, {}, rayon_par_iter_{}", &d, threads, "Multiply");
    let duration = benchmark_rayon_par_map(size, rounds);
    result.entry(key).or_insert(duration);

    result
}
