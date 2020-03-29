use std::collections::HashMap;
use std::fmt::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use chrono::{NaiveDate, NaiveTime};
use csv::StringRecord;
use rayon::prelude::*;
use serde::export::Formatter;

use crate::primitive::par_map_v1;

struct Record {
    date: NaiveDate,
    time: NaiveTime,
    recs: [i32; 55],
}

impl Record {
    #[allow(dead_code)]
    fn new() -> Record {
        Record {
            date: NaiveDate::from_ymd(1970, 12, 1),
            time: NaiveTime::from_hms_milli(00, 00, 00, 00),
            recs: [0; 55],
        }
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Record")
            .field("date", &self.date)
            .field("time", &self.time)
            .field("others", &(self.recs.to_vec()))
            .finish()
    }
}

#[allow(irrefutable_let_patterns, dead_code)]
pub fn big_map_seq(rounds: usize, threads: usize) -> HashMap<String, Duration> {
    let filename = "DEBS2012-cleaned-v3.txt.small";
    println!("Starting bm");
    let file= File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: &mut Vec<String> = &mut reader.lines().filter_map(|line|line.ok().and_then(|l|l.parse().ok())).collect();
    println!("Finished reading file");
    let lc: usize = lines.len();
    let mut m: HashMap<String, Duration> = HashMap::new();
    let seq_now = Instant::now();
    {
        println!("Running seq");
        for _ in 0..rounds {
            // sequential
            let mut res: Vec<Record> = Vec::with_capacity(lines.len());
            for line_st in lines.as_mut_slice() {
                let line: Vec<&str> = line_st.split_whitespace().collect();
                let date = NaiveDate::parse_from_str((&line)[0], "%Y-%m-%d").unwrap();
                let time = NaiveTime::parse_from_str((&line)[1], "%H:%M:%S%.f").unwrap();
                let mut arr = [0; 55];
                for i in 2..57 {
                    arr[i - 2] = (&line)[i].parse::<i32>().unwrap();
                }
                res.push(Record {
                    date,
                    time,
                    recs: arr,
                })
            }
        }
    }
    m.entry(format!("{}, {}, big_map_seq", lc, threads)).or_insert(seq_now.elapsed().div_f64(rounds as f64));

    let rayon_now = Instant::now();
    println!("Running rayon");
    {
        // parallel rayon
        for _ in 0..rounds {
            let _r: Vec<Record> = lines.as_mut_slice().into_par_iter()
                .map(|line_st: &mut String| {
                    let line: Vec<&str> = line_st.split_whitespace().collect();
                    let date = NaiveDate::parse_from_str(line[0], "%Y-%m-%d").unwrap();
                    let time = NaiveTime::parse_from_str(line[1], "%H:%M:%S%.f").unwrap();
                    let mut arr = [0; 55];
                    for i in 2..57 {
                        arr[i - 2] = line[i].parse::<i32>().unwrap();
                    }
                    Record {
                        date,
                        time,
                        recs: arr,
                    }
                }).collect();
        }
    };
    m.entry(format!("{}, {}, big_map_rayon", lc, threads)).or_insert(rayon_now.elapsed().div_f64(rounds as f64));

    let par_now = Instant::now();
    println!("Running par");
    {
        // parallel map
        for _ in 0..rounds {
            let _r: Vec<Record> = par_map_v1(
                &lines,
                &|_, line_st: &String| {
                    let line: Vec<&str> = line_st.split_whitespace().collect();
                    let date = NaiveDate::parse_from_str(line[0], "%Y-%m-%d").unwrap();
                    let time = NaiveTime::parse_from_str(line[1], "%H:%M:%S%.f").unwrap();
                    let mut arr = [0; 55];
                    for i in 2..57 {
                        arr[i - 2] = line[i].parse::<i32>().unwrap();
                    }
                    Record {
                        date,
                        time,
                        recs: arr,
                    }
                });
            println!("{:?}", _r);
        }
    };
    m.entry(format!("{}, {}, big_map_par", lc, threads)).or_insert(par_now.elapsed().div_f64(rounds as f64));
    m
}