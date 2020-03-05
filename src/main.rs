use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

use crate::primitive::*;
use crate::util::file_reader::read_csv;

pub mod util;
pub mod primitive;

fn small_compute(i: usize, e: &i32) -> i32 {
    *e + *e
}

fn fac(i: &i32) -> i32 {
    if (*i) == 2 {
        2
    } else {
        (*i) * fac(&(i - 1))
    }
}

fn huge_compute(i: usize, e: &i32) -> i32 {
    fac(e)
}

fn benchmark_v1<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &i32) -> i32)
{
    let v = read_csv(file);
    let now = Instant::now();
    par_map_v1(&v, func);
    now.elapsed()
}

fn benchmark_v2<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &i32) -> i32)
{
    let v = read_csv(file);
    let now = Instant::now();
    par_map_v2(&v, func);
    now.elapsed()
}

fn main() {
    let files = vec!["../data_small.csv", "../data_medium.csv", "../data_large.csv"];
    let mut func: HashMap<&str, &dyn Fn(usize, &i32) -> i32> = HashMap::new();
    func.insert("Small_C", &small_compute);
    func.insert("Huge_C", &huge_compute);

    let mut result: HashMap<String, Duration> = HashMap::new();
    for (&name, &f) in &func {
        for d in files {
            let key = format!("{},{},1", name, d);
            let dur = benchmark_v1(d, f);
            result.entry(key).or_insert(dur);

            let key2 = format!("{},{},2", name, d);
            let dur = benchmark_v2(d, f);
            result.entry(key2).or_insert(dur);
        }
    }
    println!("{:?}", result);
}
