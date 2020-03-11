extern crate rayon;

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, LineWriter, Write};
use std::ops::Range;
use std::path::Path;

use csv::Writer;
use rand::*;
use rayon::prelude::*;
use std::env::current_dir;

fn random_list_generator(size: u64, min: i32, max: i32) -> Vec<String>{
    let mut rng = rand::thread_rng();
    ((0..size).into_iter().map(|_|rng.gen_range(min, max).to_string()).collect())
}

pub fn make_data(size: u64, min: i32, max: i32, path: &str, type_t: &str) {
    path.to_string().retain(|x| x != '/');
    let data: Vec<String> = random_list_generator(size, min, max);
    let fname = format!("{}/{}/size-{}.csv", path, type_t, size);
    create_dir_all(format!("{:?}/{:?}", path, type_t));
    let mut f = File::create(Path::new(fname.as_str())).expect("Unable to create file");
    let mut writer = LineWriter::new(&f);
    let to_write = data.join("\n");
    write!(&mut writer, "{}", to_write);
}

pub fn make_flatten_data(size: u64, min: i32, max: i32, path: &str) {
    path.to_string().retain(|x| x != '/');
    let mut rng = rand::thread_rng();
    let mut data: Vec<Vec<String>> = Vec::with_capacity(size as usize);
    unsafe {data.set_len(size as usize);}
    let fname = format!("{}/flatten/size-{}.csv", path, size);
    let mut f = File::create(Path::new(fname.as_str())).expect("Unable to create file");
    let mut writer = LineWriter::new(&f);

    for _ in 0..data.len() {
        let dsize: u64 = rng.gen_range(3, 300);
        let line = random_list_generator(dsize, min, max);
        write!(&mut writer, "{}\n", line.join(","));
    }
}