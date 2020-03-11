use crate::benchmark::{run_map_benchmark, run_flatten_benchmark};
use std::process::exit;
use std::fs;
use std::fs::File;
use serde_json::*;
use util::file_reader::*;
use std::collections::HashMap;
use std::time::Duration;

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
//    let map_files: Vec<String> = get_files("data/map");
//    let flatten_files: Vec<String> = get_files("data/flatten");
//    if map_files.is_empty()  && flatten_files.is_empty() {
//        println!("No data to be testing on, put .csv files in data/");
//        exit(-1);
//    }
//    let _ = fs::create_dir("output/");
//    // Map
//    for d in map_files.iter() {
//        let v: Vec<u128> = read_csv(&d);
//        let map_res = run_map_benchmark(d, v);
//        let _ = serde_json::to_writer(
//            &File::create("output/map_result.json").unwrap(), &json!(map_res));
//    }
//    // Flatten
//    for d in flatten_files.iter() {
//        let v: Vec<Vec<u128>> = read_nested::<u128>(&d);
//        let v_r: Vec<&Vec<u128>> = v.iter().map(|f| f).collect();
//        let (flat_dur, flat) = run_flatten_benchmark(d, &v_r);
//        println!("{:?}", flat);
//        let _ = serde_json::to_writer(
//            &File::create("output/flatten_result.json").unwrap(), &json!(flat_dur));
//    }

    use crate::primitive::par_filter_v2;
    let arr: Vec<i32> = vec![1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1];
    let actual2: Vec<i32> = par_filter_v2(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });

    let expected: Vec<i32> = vec![1, 1];
    println!("actual2={:?}, expected={:?}", actual2, expected);
    assert_eq!(actual2, expected);
}