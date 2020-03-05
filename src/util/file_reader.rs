extern crate csv;
extern crate num;

use num::{PrimInt, Unsigned};
use std::iter::Product;
use std::str::FromStr;

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