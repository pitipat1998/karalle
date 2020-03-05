use std::collections::HashMap;
use std::{fs,io};
use std::time::{Duration, Instant};

use crate::primitive::*;
use crate::util::file_reader::read_csv;
use std::path::{PathBuf, Path};
use std::fmt::Error;
use std::process::exit;

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
        (*i) * fac(&(i - 1))
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
    let v = read_csv(file);
    let now = Instant::now();
    par_map_v2(&v, func);
    now.elapsed()
}

fn main() {
    let files: Vec<String> = fs::read_dir("data").unwrap()
        .into_iter()
        .map(|res| {
            res.unwrap().path().into_os_string()
        })
        .filter(|e|{
            e.to_os_string().into_string().unwrap().ends_with(".csv")
        })
        .map(|e|e.into_string().unwrap())
        .collect();
    if files.is_empty() {
        println!("No data to be testing on, put .csc files in data/");
        exit(-1);
    }
    let mut func: HashMap<&str, &(dyn Sync + Send + Fn(usize, &u128) -> u128)> = HashMap::new();
    func.insert("Small_C", &|i, x| { *x * *x });
    func.insert("Huge_C", &|i, x| { fac(x) });

    let mut result: HashMap<String, Duration> = HashMap::new();
    for (&name, &f) in &func {
        for d in files.iter() {
            let key = format!("{},{},1", name, &d);
            let dur = benchmark_v1(&d, f);
            result.entry(key).or_insert(dur);

            let key2 = format!("{},{},2", name,& d);
            let dur = benchmark_v2(&d, f);
            result.entry(key2).or_insert(dur);
        }
    }
    println!("{:?}", result);
}
