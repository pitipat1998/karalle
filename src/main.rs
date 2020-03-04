extern crate csv;

use std::io;
use std::time::{Duration, Instant};

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<i32> = Vec::new();

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap();
        let number = record.get(0).unwrap().parse::<i32>().unwrap();
        v.push(number)
    }
    println!("{:?}", v);
    let now = Instant::now();

}