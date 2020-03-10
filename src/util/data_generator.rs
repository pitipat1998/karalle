extern crate rayon;

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, LineWriter, Write};
use std::ops::Range;
use std::path::Path;

use csv::Writer;
use rand::*;
use rayon::prelude::*;
use std::env::current_dir;

pub fn make_data(size: u64, min: i32, max: i32, path: &str, type_t: &str) {
    path.to_string().retain(|x| x != '/');
    let mut rng = rand::thread_rng();
    let data: Vec<String> =((0..size).into_iter().map(|_|rng.gen_range(min, max).to_string()).collect());
    let fname = format!("{}/{}/size-{}.csv", path, type_t, size);
    println!("file name: {:?}", fname);
    println!("Curr: {:?}", current_dir().unwrap());
    create_dir_all(format!("{:?}/{:?}", path, type_t));
    let mut f = File::create(Path::new(fname.as_str())).expect("Unable to create file");
    let mut writer = LineWriter::new(&f);
    let to_write = data.join("\n");
    write!(&mut writer, "{}", to_write);
}