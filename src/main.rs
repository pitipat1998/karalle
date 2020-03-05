use std::io;
use std::time::{Duration, Instant};

pub mod util;
pub mod primitive;
use crate::util::file_reader::read_csv;
use crate::primitive::par_map;

fn main() {
    let v: Vec<i32> = read_csv("../data_medium.csv");
    let f= |_j: usize, a: &i32| -> i32 { if *a <= 2 { 1 } else { 0 } };
    let now = Instant::now();
    let o = par_map(&v, f);
    let stop = now.elapsed();
    println!("Time used: {:?} ns", stop.as_nanos())
}
