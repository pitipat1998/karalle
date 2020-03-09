use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::*;

#[allow(dead_code)]
fn fac(i: &u128) -> u128 {
    if (*i) <= 1 {
        1
    } else {
        let f = &fac(&(*i - 1));
        (*i) * (*f)
    }
}

// fn huge_compute(i: usize, e: &u128) -> u128 {
//     fac(e)
// }
const ROUNDS: u32 = 10;

#[allow(dead_code)]
fn benchmark_map<T, V, K>(vec: &Vec<T>, func: V, map: K) -> Duration
    where V: Sync + Send + Copy + (Fn(usize, &u128) -> u128),
          K: Sync + Send + (Fn(&Vec<T>, V) -> Vec<T>)
{
    let now = Instant::now();
    for i in 0..ROUNDS {
        map(&vec, func.clone());
    }
    now.elapsed().div_f32(ROUNDS as f32)
}

type MapFunc = (dyn Sync + Send + Fn(usize, &u128) -> u128);

#[allow(dead_code)]
pub fn run_map_benchmark(d: &String, v: Vec<u128>) -> HashMap<String, Duration> {
    let mut func: HashMap<&str, &MapFunc> = HashMap::new();
    func.insert("Multiply", &|_, x| { *x * *x });
    func.insert("Fac", &|_, x| { fac(x) });

    // let mut par_map_zip: HashMap<&str, &dyn Fn(&Vec<u128>, u128) -> Vec<u128>> = HashMap::new();
    // par_map_zip.insert("v1", &par_map_v1);
    // par_map_zip.insert("v2", &par_map_v2);
    // par_map_zip.insert("v3", &par_map_v3);

    let mut result: HashMap<String, Duration> = HashMap::new();

    for (&fname, &f) in &func {
        // let key = format!("{}, {}, seq_map", fname, &d);
        // let duration = benchmark_map(&v, f, map);
        // result.entry(key).or_insert(duration);

        let key = format!("{}, {}, sqrt_n", fname, &d);
        let duration = benchmark_map(&v, f, par_map_v1);
        result.entry(key).or_insert(duration);

        // let key = format!("{}, {}, n_spawn", fname, &d);
        // let duration = benchmark_map(&v, f, par_map_v2);
        // result.entry(key).or_insert(duration);

        let key = format!("{}, {}, half_split", fname, &d);
        let duration = benchmark_map(&v, f, par_map_v3);
        result.entry(key).or_insert(duration);

        // let key = format!("{}, {}, 4nproc", fname, &d);
        // let duration = benchmark_map(&v, f, par_map_v4);
        // result.entry(key).or_insert(duration);

        let key = format!("{}, {}, rayon_par_iter", fname, &d);
        let duration = benchmark_map(&v, f, par_map_v5);
        result.entry(key).or_insert(duration);
    }
    result
}
