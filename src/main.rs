use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::time::{Duration, Instant};

use serde_json::json;

use crate::primitive::*;
use crate::util::file_reader::read_csv;

pub mod util;
pub mod primitive;

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
fn benchmark_map<T, V, K>(vec: &Vec<T>, func: V, map: K) -> Duration
    where V: Sync + Send + (Fn(usize, &u128) -> u128),
          K: Sync + Send + (Fn(&Vec<T>, V) -> Vec<T>)
{
    let now = Instant::now();
    map(&vec, func);
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
        println!("No data to be testing on, put .csv files in data/");
        exit(-1);
    }

    let mut func: HashMap<&str, &(dyn Sync + Send + Fn(usize, &u128) -> u128)> = HashMap::new();
    func.insert("Multiply", &|_, x| { *x * *x });
    func.insert("Fac", &|_, x| { fac(x) });

    // let mut par_map_zip: HashMap<&str, &dyn Fn(&Vec<u128>, u128) -> Vec<u128>> = HashMap::new();
    // par_map_zip.insert("v1", &par_map_v1);
    // par_map_zip.insert("v2", &par_map_v2);
    // par_map_zip.insert("v3", &par_map_v3);

    let mut result: HashMap<String, Duration> = HashMap::new();
    for d in files.iter() {
        let v: Vec<u128> = read_csv(&d);
        for (&fname, &f) in &func {
            let key = format!("{}, {}, sqrt_n", fname, &d);
            let duration = benchmark_map(&v, f, par_map_v1);
            result.entry(key).or_insert(duration);

            let key = format!("{}, {}, n_spawn", fname, &d);
            let duration = benchmark_map(&v, f, par_map_v2);
            result.entry(key).or_insert(duration);

            let key = format!("{}, {}, par_iter", fname, &d);
            let duration = benchmark_map(&v, f, par_map_v3);
            result.entry(key).or_insert(duration);

            let key = format!("{}, {}, 4nproc", fname, &d);
            let duration = benchmark_map(&v, f, par_map_v4);
            result.entry(key).or_insert(duration);
        }
    }
    let s = json!(result);
    let _ = fs::create_dir("output/");
    let _ = serde_json::to_writer(&File::create("output/data.json").unwrap(), &s);
    // println!("{:?}", result);
}
