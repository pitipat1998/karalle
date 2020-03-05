use std::fs;
use std::collections::HashMap;
use std::process::exit;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::primitive::*;
use crate::util::file_reader::read_csv;

pub mod util;
pub mod primitive;
//
// fn small_compute(i: usize, e: &u128) -> u128 {
//     *e + *e
// }

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

fn benchmark_v1<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &u128) -> u128)
{
    let v = read_csv(file);
    let now = Instant::now();
    par_map_v1(&v, func);
    now.elapsed()
}

fn benchmark_v2<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &u128) -> u128)
{
    let v: Vec<u128> = read_csv(file);
    let now = Instant::now();
    par_map_v2(&v, func);
    now.elapsed()
}

fn benchmark_v3<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &u128) -> u128)
{
    let v: Vec<u128> = read_csv(file);
    let now = Instant::now();
    par_map_v3(&v, func);
    now.elapsed()
}

fn benchmark_v4<V>(file: &str, func: V) -> Duration
    where V: Sync + Send + (Fn(usize, &u128) -> u128)
{
    let v: Vec<u128> = read_csv(file);
    let now = Instant::now();
    par_map_v4(&v, func);
    now.elapsed()
}

fn get_files() -> Vec<String> {
    fs::read_dir("data").unwrap()
        .into_iter()
        .map(|res| {
            res.unwrap().path().into_os_string()
        })
        .filter(|e| {
            e.to_os_string().into_string().unwrap().ends_with(".csv")
        })
        .map(|e| e.into_string().unwrap())
        .collect()
}

fn main() {
    let files: Vec<String> = get_files();
    if files.is_empty() {
        println!("No data to be testing on, put .csc files in data/");
        exit(-1);
    }
    let mut func: HashMap<&str, &(dyn Sync + Send + Fn(usize, &u128) -> u128)> = HashMap::new();
    func.insert("Multiply", &|_, x| { *x * *x });
    func.insert("Fac", &|_, x| { fac(x) });

    let mut result: HashMap<String, Duration> = HashMap::new();
    for (&name, &f) in &func {
        for d in files.iter() {
            let key = format!("{}, {}, v1", name, &d);
            let dur = benchmark_v1(&d, f);
            result.entry(key).or_insert(dur);

            let key2 = format!("{}, {}, v2", name, &d);
            let dur = benchmark_v2(&d, f);
            result.entry(key2).or_insert(dur);

            let key3 = format!("{}, {}, v3", name, &d);
            let dur = benchmark_v3(&d, f);
            result.entry(key3).or_insert(dur);

            let key4 = format!("{}, {}, v4", name, &d);
            let dur = benchmark_v4(&d, f);
            result.entry(key4).or_insert(dur);
        }
    }
    println!("{:?}", result);
}
