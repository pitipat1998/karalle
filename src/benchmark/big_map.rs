use std::collections::HashMap;
use std::fmt::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use chrono::{NaiveDate, NaiveTime};
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

fn run_seq(rounds: usize, buffer_vec: &mut Vec<String>) -> Duration {
    println!("Running seq");
    let seq_now = Instant::now();
    for _ in 0..rounds {
        // sequential
        let mut v: Vec<Record> = Vec::new();
        for line_st in buffer_vec.as_mut_slice() {
            let line: Vec<&str> = line_st.split_whitespace().collect();
            let date = NaiveDate::parse_from_str((&line)[0], "%Y-%m-%d").unwrap();
            let time = NaiveTime::parse_from_str((&line)[1], "%H:%M:%S%.f").unwrap();
            let mut arr = [0; 55];
            for i in 2..57 {
                arr[i - 2] = (&line)[i].parse::<i32>().unwrap();
            }
            v.push(Record{date, time, recs: arr});
        }
    }
    seq_now.elapsed()
}

fn run_rayon(rounds: usize, buffer_vec: &mut Vec<String>) -> Duration {
    // parallel rayon
    let ray_now = Instant::now();
    for _ in 0..rounds {
        let _r: Vec<Record> = buffer_vec.as_mut_slice().into_par_iter()
            .map(|line_st: &mut String| {
                let line: Vec<&str> = line_st.split_whitespace().collect();
                let date = NaiveDate::parse_from_str(line[0], "%Y-%m-%d").unwrap();
                let time = NaiveTime::parse_from_str(line[1], "%H:%M:%S%.f").unwrap();
                let mut arr = [0; 55];
                for i in 2..57 {
                    arr[i - 2] = line[i].parse::<i32>().unwrap();
                }
                Record{date, time, recs: arr}
            }).collect();
    }
    ray_now.elapsed()
}

fn run_par(rounds: usize, buffer_vec: &mut Vec<String>) -> Duration {
    // parallel map
    let par_now = Instant::now();
    for _ in 0..rounds {
        let _r: Vec<Record> = par_map_v1(
            &buffer_vec,
            &|_, line_st: &String| {
                let line: Vec<&str> = line_st.split_whitespace().collect();
                let date = NaiveDate::parse_from_str(line[0], "%Y-%m-%d").unwrap();
                let time = NaiveTime::parse_from_str(line[1], "%H:%M:%S%.f").unwrap();
                let mut arr = [0; 55];
                for i in 2..57 {
                    arr[i - 2] = line[i].parse::<i32>().unwrap();
                }
                Record{date, time, recs: arr}
            });
    }
    par_now.elapsed()
}

#[allow(irrefutable_let_patterns, dead_code)]
pub fn big_map_seq(rounds: usize, threads: usize) -> HashMap<String, Duration> {
    let filename = "DEBS2012-cleaned-v3.txt.small";
    println!("Starting bm");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut m: HashMap<String, Duration> = HashMap::new();
    let lines = reader.lines();
    let mut c = 0;
    let mut lc = 0;
    let mut buffer_vec: Vec<String> = Vec::new();
    let mut t_seq = Duration::new(0, 0);
    let mut t_ray = Duration::new(0, 0);
    let mut t_par = Duration::new(0, 0);
    for lr in lines.into_iter() {
        lc += 1;
        match lr {
            Ok(line) => {
                if c == 10_000 {
                    t_seq += run_seq(rounds, &mut buffer_vec);
                    t_ray += run_rayon(rounds, &mut buffer_vec);
                    t_par += run_par(rounds, &mut buffer_vec);
                    buffer_vec.clear();
                }
                buffer_vec.push(line);
                c += 1;
            }
            Err(_) => {}
        }
    }
    t_seq += run_seq(rounds, &mut buffer_vec);
    t_ray += run_rayon(rounds, &mut buffer_vec);
    t_par += run_par(rounds, &mut buffer_vec);

    m.entry(format!("{}, {}, big_map_seq", lc, threads)).or_insert(t_seq.div_f64(rounds as f64));
    m.entry(format!("{}, {}, big_map_rayon", lc, threads)).or_insert(t_ray.div_f64(rounds as f64));
    m.entry(format!("{}, {}, big_map_par", lc, threads)).or_insert(t_par.div_f64(rounds as f64));

    m
}