extern crate envmnt;
extern crate num_cpus;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::time::Duration;

use serde_json::*;

use util::data_generator::*;
use util::file_reader::*;

use crate::benchmark::{run_flatten_benchmark, run_map_benchmark};

pub mod util;
pub mod benchmark;
pub mod primitive;

fn get_files(dir: &str) -> Vec<String> {
    fs::read_dir(dir).unwrap()
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
    let make_type = envmnt::get_or("KGEN", "none").to_lowercase();
    match make_type.as_str() {
        "map" | "filter" => {
            (20..41).for_each(|i| {
                let size = (2 as f32).powi(i) as u64;
                println!("Generating {} {} data size", size, make_type);
                make_data(size, 2, 1000, "data", make_type.as_str());
                println!("Done");
            })
        }
        "flatten" => {
            (20..41).for_each(|i| {
                let size = (2 as f32).powi(i) as u64;
                println!("Generating {} flatten data size", size);
                make_flatten_data(size, 2, 1000, "data");
                println!("Done");
            })
        }
        "all" => {
            for t in ["filter", "map"].iter() {
                (20..41).for_each(|i| {
                    let size = (2 as f32).powi(i) as u64;
                    println!("Generating {} {} data size", size, t);
                    make_data(size, 2, 1000, "data", t);
                    println!("Done");
                })
            }
            (20..41).for_each(|i| {
                let size = (2 as f32).powi(i) as u64;
                println!("Generating {} flatten data size", size);
                make_flatten_data(size, 2, 1000, "data");
                println!("Done");
            })
        }
        _ => println!("Usage: KGEN=<map|filter|flatten|all>")
    }
    if make_type != "none" { return; }

    let mut tn: usize = envmnt::get_or("KTHREAD", "0").parse().unwrap();
    if tn == 0 {
        tn = num_cpus::get();
    }
    rayon::ThreadPoolBuilder::new().num_threads(tn).build_global().unwrap();

    let r = envmnt::get_or("KROUND", "10");
    let rounds: u128 = r.parse().unwrap();
    println!("Running with {} threads and {} rounds", tn, rounds);

    let t: String = envmnt::get_or("KTYPE", "ALL");

    let map_files: Vec<String> = get_files("data/map");
    let flatten_files: Vec<String> = get_files("data/flatten");
    if map_files.is_empty() && flatten_files.is_empty() {
        println!("No data to be testing on, put .csv files in data/");
        exit(-1);
    }
    let _ = fs::create_dir("output/");
    if t == "ALL" || t == "MAP" {
        let mut map_res: HashMap<String, Duration> = HashMap::new();
        // Map
        for d in map_files.iter() {
            println!("Running map file: {}", d);
            let v: Vec<u128> = read_csv(&d);
            let res = run_map_benchmark(d, v, rounds, tn);
            map_res.extend(res);
        }
        println!("Writing map result");
        let _ = serde_json::to_writer(
            &File::create(format!("output/map-T{}-R{}.json", tn, rounds)).unwrap(), &json!(map_res));
    }
    if t == "ALL" || t == "FLATTEN" {
        let mut flat_res: HashMap<String, Duration> = HashMap::new();
        // Flatten
        for d in flatten_files.iter() {
            println!("Running flatten file: {}", d);
            let v: Vec<Vec<u128>> = read_nested::<u128>(&d);
            // let v_r: Vec<&Vec<u128>> = v.iter().map(|f| f).collect();
            let res = run_flatten_benchmark(d, &v, rounds, tn);
            flat_res.extend(res);
        }
        println!("Writing flatten result");
        let _ = serde_json::to_writer(
            &File::create(format!("output/flatten-T{}-R{}.json", tn, rounds)).unwrap(), &json!(flat_res));
    }

    // let v:Vec<Vec<_>> = vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![7, 8, 9]
    // ];
    // let x: Vec<&Vec<_>> = vec![&vec![1, 2], &vec![3, 4]];
    // let mut y: Vec<Vec<i32>> = x.iter().map(|&i| i).collect();
    //
    // let z: Vec<_> = y.into_par_iter().flatten().collect();
}