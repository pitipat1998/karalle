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

use project_k::primitive::{vec_init};
use util::data_generator::*;
use crate::benchmark::*;

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
        _ => println!("Usage: KMAKE=<map|filter|flatten|all>")
    }
}

fn write_output(func: &String, result: HashMap<String, Duration>,
                rounds: u128, threads: usize,
) {
    let _ = serde_json::to_writer(
        &File::create(format!("output/{}-T{}-R{}.json", func, threads, rounds)).unwrap(), &json!(result));
}

fn main() {
    let mut max_size: usize = envmnt::get_or("KSIZE", "30").parse().unwrap();
    if max_size >= 31 {
        println!("Can't go more than 30 sorry,automatically using KSIZE=30");
        max_size = 30;
    }
    let sizes: Vec<u64> = vec_init(max_size, &|i| { (1 << (i + 1)) as u64 }, 2000);
    let make_type = envmnt::get_or("KMAKE", "none").to_lowercase();

    if &make_type != "none" {
        make_file(&make_type);
        return;
    }

    let mut tn: usize = envmnt::get_or("KTHREAD", "0").parse().unwrap();
    if tn == 0 {
        tn = num_cpus::get();
    }
    rayon::ThreadPoolBuilder::new().num_threads(tn).build_global().unwrap();

    let r = envmnt::get_or("KROUND", "100");
    let rounds: u128 = r.parse().unwrap();
    println!("Running with {} threads and {} rounds", tn, rounds);

    let t: String = envmnt::get_or("KTYPE", "all").to_lowercase();

    // let files_1d: Vec<String> = get_files("data/map");
    // let files_2d: Vec<String> = get_files("data/flatten");
    // if files_1d.is_empty()  {
    //     println!("No data to be testing on, run `KMAKE=<type> cargo run --release`");
    //     exit(-1);
    // }

    let _ = fs::create_dir("output/");

    if t == "loop" {
        let mut for_res: HashMap<String, Duration> = HashMap::new();
        for size in &sizes {
            println!("Running loop size: {}", &size);
            let res = run_loop_benchmark(&size.to_string(), *size, rounds, tn);
            for_res.extend(res);
        }
        println!("Writing loop result");
        write_output(&"loop".to_string(), for_res, rounds, tn);
    }

    if t == "all" || t == "filter" {
        let mut filter_res: HashMap<String, Duration> = HashMap::new();
        // Map
        for size in &sizes {
            println!("Running filter size: {}", &size);
            let res = run_filter_benchmark(&size.to_string(), *size , rounds, tn);
            filter_res.extend(res);
        }
        println!("Writing filter result");
        write_output(&"filter".to_string(), filter_res, rounds, tn);
    }

    if t == "all" || t == "map" {
        let mut map_res: HashMap<String, Duration> = HashMap::new();
        // Map
        for size in &sizes {
            println!("Running map size: {:?}", &size);
            let res = run_map_benchmark(&size.to_string(), *size, rounds, tn);
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

    if t == "all" || t == "flatten" {
        let mut flat_res: HashMap<String, Duration> = HashMap::new();
        for size in &sizes {
            println!("Running flatten size: {}", size);
            let res = run_flatten_benchmark(&size.to_string(), *size, rounds, tn);
            flat_res.extend(res);
        }
        println!("Writing flatten result");
        write_output(&"flatten".to_string(),flat_res, rounds, tn);
    }
    if t == "big_map" || t == "bm" {
        let bm_res = big_map_seq(rounds as usize, tn);
        println!("Writing big_map result");
        write_output(&"big_map".to_string(), bm_res, rounds, tn);
    }

    if t == "all" || t == "scan" {
        let mut scan_res: HashMap<String, Duration> = HashMap::new();
        // Scan
        for size in &sizes {
            println!("Running scan file: {}", size);
            let res = run_scan_benchmark(&size.to_string(), *size, rounds, tn);
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

