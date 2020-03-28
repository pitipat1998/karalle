extern crate csv;
extern crate num;

use std::fs::*;
use std::io;
use std::io::BufRead;
use std::iter::Product;
use std::path::Path;
use std::str::FromStr;

use num::{PrimInt};
use serde::de::DeserializeOwned;
use serde_json::from_str;

#[allow(dead_code)]
pub fn read_csv<T>(path: &str) -> Vec<T>
where T: PrimInt + Product + FromStr,
      <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let rdr = csv::Reader::from_path(path);
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<T> = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        let number = record.get(0).unwrap().parse::<T>().unwrap();
        v.push(number)
    }
    v
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let line =io::BufReader::new(file).lines();
    Ok(line)
}

#[allow(dead_code)]
pub fn read_nested<T: DeserializeOwned>(path: &str) -> Vec<Vec<T>> {
    let mut v: Vec<Vec<T>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_op in lines {
            if let Ok(line) = line_op {
                let x: Vec<T> = line.split(",").map(|i| from_str::<T>(i).unwrap()).collect();
                v.push(x);
            }
        }
    }
    v
}