extern crate envmnt;
extern crate num_cpus;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::time::{Duration};

use rayon::prelude::*;
use serde_json::*;

use util::data_generator::*;
use util::file_reader::*;

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
    let make_type = envmnt::get_or("KMAKE", "none").to_lowercase();
    make_file(&make_type);
    if &make_type != "none" { return; }

    let mut tn: usize = envmnt::get_or("KTHREAD", "0").parse().unwrap();
    if tn == 0 {
        tn = num_cpus::get();
    }
    rayon::ThreadPoolBuilder::new().num_threads(tn).build_global().unwrap();

    let r = envmnt::get_or("KROUND", "10");
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

    if t == "all" || t == "qs" || t == "quick_sort" {
        let mut qs_res: HashMap<String, Duration> = HashMap::new();
        // Quick_sort
        for d in files_1d.iter() {
            println!("Running qs file: {}", d);
            let mut v: Vec<i16> = read_csv::<i16>(&d);
            let res = run_quick_sort_benchmark(d, &mut v, |a: &i16, b: &i16| -> i32 { (*a - *b) as i32 }, rounds, tn);
            qs_res.extend(res);
        }
        println!("Writing qs result");
        write_output(&"qs".to_string(), qs_res, rounds, tn);
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
    // if t == "all" || t == "sample_sort" || t == "ss" {
    //     let mut ss_res: HashMap<String, Duration> = HashMap::new();
    //     for d in files_1d.iter() {
    //         println!("Running sample sort file: {}", d);
    //         let mut v: Vec<i32> = read_csv::<i32>(&d);
    //         let res = run_sample_sort_benchmark(d, &mut v, |a: &i32, b: &i32| -> i32 { (*a - *b) as i32 }, rounds, tn);
    //         ss_res.extend(res);
    //     }
    //     println!("Writing sample sort result");
    //     write_output(&"sample_sort".to_string(), ss_res, rounds, tn);
    // }
    // let v:Vec<Vec<_>> = vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![7, 8, 9]
    // ];
    // let x: Vec<&Vec<_>> = vec![&vec![1, 2], &vec![3, 4]];
    // let mut y: Vec<Vec<i32>> = x.iter().map(|&i| i).collect();
    //
    // let z: Vec<_> = y.into_par_iter().flatten().collect();

//    use crate::sort::*;
//    use crate::primitive::*;
//    use rayon::prelude::*;
//
//    let mut rng = rand::thread_rng();
//    let tim = Instant::now();
//    for i in 0..10 {
//        let mut arr1: Vec<i32> = random_i32_list_generator(rng.gen_range(3000000, 3000001), -10, 11);
////        par_map_v3(&mut arr1, &|a: usize, b: &i32| { 2*b });
//        par_quick_sort_v2(&mut arr1, &|a: &i32, b: &i32| { *a - *b });
//    }
//    println!("inplace time={}", tim.elapsed().as_secs_f64()/10.0);
//
//    let tim = Instant::now();
//    for i in 0..10 {
//        let mut arr1: Vec<i32> = random_i32_list_generator(rng.gen_range(3000000, 3000001), -10, 11);
////        par_map_v3(&mut arr1, &|a: usize, b:&i32| { 2*b });
//        par_quick_sort_v3(&mut arr1, &|a: &i32, b: &i32| { *a - *b });
//    }
//    println!("rayon time={}", tim.elapsed().as_secs_f64()/10.0);
}