extern crate envmnt;
extern crate num_cpus;
extern crate chrono;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::time::Duration;

use rayon::prelude::*;
use serde_json::*;

use project_k::primitive::{vec_init, par_map_v5};
use util::data_generator::*;
use util::file_reader::*;

use crate::benchmark::*;
use crate::primitive::par_map_v3;

pub mod util;
pub mod benchmark;
pub mod primitive;
pub mod sort;
pub mod constant;

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

fn make_file(make_type: &String) {
    match make_type.as_str() {
        "map" | "filter" => {
            (20..30).into_par_iter()
                .for_each(|i| {
                    let size = (2 as f32).powi(i) as u64;
                    println!("Generating {}({}) {} data size", size, i, make_type);
                    make_data(size, 2, 1000, "data", make_type.as_str());
                    println!("Done");
                });
            return;
        }
        "flatten" => {
            (20..24).into_par_iter()
                .for_each(|i| {
                    let size = (2 as f32).powi(i) as u64;
                    println!("Generating {}({}) flatten data size", size, i);
                    make_flatten_data(size, 2, 1000, "data");
                    println!("Done");
                });
            return;
        }
        "all" => {
            for t in ["filter", "map"].iter() {
                (20..30).into_par_iter()
                    .for_each(|i| {
                        let size = (2 as f32).powi(i) as u64;
                        println!("Generating {}({}) {} data size", size, i, t);
                        make_data(size, 2, 1000, "data", t);
                        println!("Done");
                    });
            }
            (20..30).into_par_iter()
                .for_each(|i| {
                    let size = (2 as f32).powi(i) as u64;
                    println!("Generating {}({}) flatten data size", size, i);
                    make_flatten_data(size, 2, 1000, "data");
                    println!("Done");
                });
            return;
        }
        _ => println!("Usage: KGEN=<map|filter|flatten|all>")
    }
}

fn write_output(func: &String, result: HashMap<String, Duration>,
                rounds: u128, threads: usize,
) {
    let _ = serde_json::to_writer(
        &File::create(format!("output/{}-T{}-R{}.json", func, threads, rounds)).unwrap(), &json!(result));
}

fn main() {
    let mut max_size: usize = envmnt::get_or("KSIZE", "27").parse().unwrap();
    let sizes: Vec<u64> = vec_init(max_size, &|i, _| { (1 << (i + 1)) as u64 }, 2000);
    let make_type = envmnt::get_or("KMAKE", "none").to_lowercase();

    if &make_type != "none" { return; }
    else {make_file(&make_type);}

    let mut tn: usize = envmnt::get_or("KTHREAD", "0").parse().unwrap();
    if tn == 0 {
        tn = num_cpus::get();
    }
    rayon::ThreadPoolBuilder::new().num_threads(tn).build_global().unwrap();

    let r = envmnt::get_or("KROUND", "100");
    let rounds: u128 = r.parse().unwrap();
    println!("Running with {} threads and {} rounds", tn, rounds);

    let t: String = envmnt::get_or("KTYPE", "ALL").to_lowercase();

    let files_1d: Vec<String> = get_files("data/map");
    let files_2d: Vec<String> = get_files("data/flatten");
    if files_1d.is_empty() && files_2d.is_empty() {
        println!("No data to be testing on, put .csv files in data/");
        exit(-1);
    }
    let _ = fs::create_dir("output/");

    if t == "all" || t == "map" {
        let mut map_res: HashMap<String, Duration> = HashMap::new();
        // Map
        for d in files_1d.iter() {
            println!("Running map file: {}", d);
            let v: Vec<u128> = read_csv(&d);
            let res = run_map_benchmark(d, v, rounds, tn);
            map_res.extend(res);
        }
        println!("Writing map result");
        write_output(&"map".to_string(), map_res, rounds, tn);
    }

    if t == "all" || t == "sort" {
        let mut sort_res: HashMap<String, Duration> = HashMap::new();
        for size in &sizes {
            println!("Running sorting size: {}", size);
            let res = run_sorting_benchmark(&size.to_string(), *size, rounds, tn);
            sort_res.extend(res);
        }
        println!("Writing sort result");
        write_output(&"sort".to_string(), sort_res, rounds, tn);
    }
//    if t == "all" || t == "flatten" {
//        let mut flat_res: HashMap<String, Duration> = HashMap::new();
//        // Flatten
//        for d in files_2d.iter() {
//            println!("Running flatten file: {}", d);
//            let v: Vec<Vec<u32>> = read_nested::<u32>(&d);
//            // let v_r: Vec<&Vec<u128>> = v.iter().map(|f| f).collect();
//            let res = run_flatten_benchmark(d, &v, rounds, tn);
//            flat_res.extend(res);
//        }
//        println!("Writing flatten result");
//        write_output(&t,flat_res, rounds, tn);
//    }
    if t == "big_map" || t == "bm" {
        let bm_res = big_map_seq(rounds as usize, tn);
        println!("Writing big_map result");
        write_output(&"big_map".to_string(), bm_res, rounds, tn);
    }

    if t == "all" || t == "scan" {
        let mut scan_res: HashMap<String, Duration> = HashMap::new();
        // Scan
        for d in files_1d.iter() {
            println!("Running scan file: {}", d);
            let mut v: Vec<i32> = read_csv::<i32>(&d);
            let res = run_scan_benchmark(d, &mut v, rounds, tn);
            scan_res.extend(res);
        }
        println!("Writing scan result");
        write_output(&"scan".to_string(), scan_res, rounds, tn);
    }

    if t == "qs" || t == "quick_sort" {
        let mut qs_res: HashMap<String, Duration> = HashMap::new();
        // Quick_sort
        for size in &sizes {
            println!("Running qs size: {}", size);
            let res = run_quick_sort_benchmark(&size.to_string(), *size, rounds, tn);
            qs_res.extend(res);
        }
        println!("Writing qs result");
        write_output(&"qs_thd".to_string(), qs_res, rounds, tn);
    }

    if t == "sample_sort" || t == "ss" {
        let mut ss_res: HashMap<String, Duration> = HashMap::new();
        for size in &sizes {
            println!("Running sample sort size: {}", size);
            let res = run_sample_sort_benchmark(&size.to_string(), *size, rounds, tn);
            ss_res.extend(res);
        }
        println!("Writing sample sort result");
        write_output(&"sample_sort".to_string(), ss_res, rounds, tn);
    }

    if t == "ms" || t == "mergesort" {
        let mut ms_res: HashMap<String, Duration> = HashMap::new();
        for size in &sizes {
            println!("Running ms size: {}", size);
            let res = run_merge_sort_benchmark(&size.to_string(), *size, rounds, tn);
            ms_res.extend(res);
        }
        println!("Writing merge_sort result");
        write_output(&"merge_sort".to_string(), ms_res, rounds, tn);
    }

//     if t == "fronk" {
//         use std::time::*;
//         use crate::sort::*;
//         for size in sizes {
//             println!("size={}", size);
//             let mut tot_time = Duration::new(0, 0);
//             for _ in 0..rounds {
//                 let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
//                 let t = Instant::now();
//                 par_quick_sort_v2(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
//                 tot_time += t.elapsed();
//             }
//             println!("par qs in-place: {}", tot_time.div_f64(rounds as f64).as_secs_f64());
//             let mut tot_time = Duration::new(0, 0);
//             for _ in 0..rounds {
//                 let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
//                 let t = Instant::now();
//                 par_sample_sort(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
//                 tot_time += t.elapsed();
//             }
//             println!("par ss in-place: {}", tot_time.div_f64(rounds as f64).as_secs_f64());
//             let mut tot_time = Duration::new(0, 0);
//             for _ in 0..rounds {
//                 let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
//                 let t = Instant::now();
//                 par_quick_sort_v3(&mut arr, &|a: &i16, b: &i16| -> i32 { (*a - *b) as i32 });
//                 tot_time += t.elapsed();
//             }
//             println!("par qs rayon: {}", tot_time.div_f64(rounds as f64).as_secs_f64());
//             println!();
//         }
//     }
}

