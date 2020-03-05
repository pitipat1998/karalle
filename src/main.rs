extern crate csv;

use std::io;
use std::time::{Duration, Instant};

mod primitive;
use crate::primitive::par_map;
fn main() {
//    let mut rdr = csv::Reader::from_reader(io::stdin());
//    let mut v: Vec<i32> = Vec::new();
//
//    for result in rdr.records() {
//        // The iterator yields Result<StringRecord, Error>, so we check the
//        // error here.
//        let record = result.unwrap();
//        let number = record.get(0).unwrap().parse::<i32>().unwrap();
//        v.push(number)
//    }
//    println!("{:?}", v);
//    let now = Instant::now();
//

    let arr: &Vec<i32> = &vec![61, 81, 50, 59, 7, 31, 11, 36, 93, 15, 36, 72, 96, 34, 2, 32, 83,
                               24, 81, 76, 22, 60, 9, 54, 72, 13, 90, 75, 47, 7, 7, 17, 68, 90,
                               86, 32, 54, 67, 50, 69, 93, 89, 30, 47, 99, 73, 18, 74, 49, 77, 53,
                               40, 70, 65, 35, 53, 19, 73, 52, 14, 93, 66, 71, 87, 72, 90, 12, 12,
                               81, 75, 79, 18, 63, 46, 40, 92, 31, 94, 64, 94, 8, 1, 4, 44, 5,
                               57, 66, 67, 9, 75, 9, 49, 61, 68, 11, 25, 39, 90, 86, 48, 91];

    let actual: Vec<i32> = par_map(arr, |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });

    let expected: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
                                  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                  0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                  0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0,
                                  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

    println!("actual={:?}, expected={:?}", actual, expected);

}
