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
    let _ = fs::create_dir("output/");
    for d in files.iter() {
        let v: Vec<u128> = read_csv(&d);
        let map_res = run_map_benchmark(d, v);
        let _ = serde_json::to_writer(
            &File::create("output/map_result.json").unwrap(), &json!(map_res));

        let arr:  & Vec<i32> = &vec![1, 2, 3, 4];
        let arr2: & Vec<i32> = & vec![5, 6, 7, 8];
        let arr3: & Vec<i32> = & vec![9, 10, 11, 12];
        let vv = vec![
            arr,
            arr2,
            arr3,
        ];
        println!("input: {:?}", vv);
        let (flat_dur, flat) : (HashMap<String, Duration>, Vec<i32>) = run_flatten_benchmark(d, &vv);
        println!("{:?}", flat);
        let _ = serde_json::to_writer(
            &File::create("output/flatten_result.json").unwrap(), &json!(flat_dur));

    }

}