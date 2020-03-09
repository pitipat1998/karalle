use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::primitive::*;

const ROUNDS: u32 = 10;

// par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
#[allow(dead_code)]
type FlattenFunc<T> = dyn Sync + Send + Fn(&Vec<&Vec<T>>) -> Vec<T>;

#[allow(dead_code)]
fn benchmark_flatten<T: Copy + Sync , K>(vec: &Vec<&Vec<T>>, flat: K) -> Duration
    where K: Sync + Send + (Fn(&Vec<&Vec<T>>) -> Vec<T>)
{
    let now = Instant::now();
    for _ in 0..ROUNDS{
        let _ = flat(&vec);
    }
    now.elapsed().div_f32(ROUNDS as f32)
}

#[allow(dead_code)]
pub fn run_flatten_benchmark<T: Copy + Sync + Send>(d: &String, v: &Vec<&Vec<T>>) -> HashMap<String, Duration> {
    let mut result: HashMap<String, Duration> = HashMap::new();
    let key = format!("{}, flatten", &d);
    let duration= benchmark_flatten(&v, par_flatten);
    result.entry(key).or_insert(duration);
    result
}