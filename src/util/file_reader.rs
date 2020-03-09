extern crate csv;
extern crate num;

use num::{PrimInt, Unsigned};
use std::iter::Product;
use std::str::FromStr;
use std::fs::*;
use std::io;
use std::path::Path;
use std::io::BufRead;
use serde_json::from_str;
use serde::de::DeserializeOwned;


#[allow(dead_code)]
pub fn read_csv<T>(path: &str) -> Vec<T>
where T: PrimInt + Unsigned + Product + FromStr,
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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