extern crate rayon;

use std::fs::{create_dir_all, File};
use std::io::{LineWriter, Write};
use std::path::Path;

use rand::*;

#[allow(dead_code)]
fn random_list_generator(size: u64, min: i32, max: i32) -> Vec<String>{
    let mut rng = rand::thread_rng();
    ((0..size).into_iter().map(|_|rng.gen_range(min, max).to_string()).collect())
}

#[allow(dead_code)]
pub fn make_data(size: u64, min: i32, max: i32, path: &str, type_t: &str) {
    path.to_string().retain(|x| x != '/');

    let fname = format!("{}/{}/size-{}.csv", path, type_t, size);
    let _ = create_dir_all(format!("{:?}/{:?}", path, type_t));
    let f = File::create(Path::new(fname.as_str())).expect("Unable to create file");
    let mut writer = LineWriter::new(&f);

    let thr = 1_000_000 as u64;
    let it = ((size/thr) as f32).floor() as u64;
    let leftover = size - (thr*it);
    for _ in 0..it {
        let data: Vec<String> = random_list_generator(thr, min, max);
        let to_write = data.join("\n");
        let _  = write!(&mut writer, "{}\n", to_write);
    }
    let data: Vec<String> = random_list_generator(leftover, min, max);
    let to_write = data.join("\n");
    let _  = write!(&mut writer, "{}", to_write);
}

#[allow(dead_code)]
pub fn make_flatten_data(size: u64, min: i32, max: i32, path: &str) {
    path.to_string().retain(|x| x != '/');
    let mut rng = rand::thread_rng();
    let mut data: Vec<Vec<String>> = Vec::with_capacity(size as usize);
    unsafe {data.set_len(size as usize);}
    let fname = format!("{}/flatten/size-{}.csv", path, size);
    let f = File::create(Path::new(fname.as_str())).expect("Unable to create file");
    let mut writer = LineWriter::new(&f);
    let _ = create_dir_all(format!("{:?}/{:?}", path, "flatten"));
    for _ in 0..data.len() {
        let dsize: u64 = rng.gen_range(3, 300);
        let line = random_list_generator(dsize, min, max);
        let _ = write!(&mut writer, "{}\n", line.join(","));
    }
}