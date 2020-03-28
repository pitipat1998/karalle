use std::collections::HashMap;
use std::fmt::*;
use std::time::{Duration, Instant};

use chrono::{NaiveDate, NaiveTime};
use csv::StringRecord;
use rayon::prelude::*;
use serde::export::Formatter;

use crate::primitive::par_map_v1;

struct Record {
    date: NaiveDate,
    time: NaiveTime,
    recs: Vec<i32>,
}

impl Record {
    #[allow(dead_code)]
    fn new() -> Record {
        Record {
            date: NaiveDate::from_ymd(1970, 12, 1),
            time: NaiveTime::from_hms_milli(00, 00, 00, 00),
            recs: Vec::new(),
        }
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Record")
            .field("date", &self.date)
            .field("time", &self.time)
            .field("others", &self.recs)
            .finish()
    }
}

#[allow(irrefutable_let_patterns, dead_code)]
pub fn big_map_seq(rounds: usize, threads: usize) -> HashMap<String, Duration> {
    let filename = "DEBS2012-cleaned-v3.txt.small";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(filename).unwrap();
    let lines: &mut Vec<StringRecord> = &mut Vec::new();

    for result in rdr.records() {
        match result {
            Ok(line) => {
                lines.push(line);
            }
            Err(e) => println!("{:?}", e)
        }
    }
    let lc = lines.clone().len();
    let mut m: HashMap<String, Duration> = HashMap::new();
    let seq_now = Instant::now();
    {
        println!("Running seq");
        for _ in 0..rounds {
            // sequential
            let mut res: Vec<Record> = Vec::with_capacity(lines.len());
            for line in lines.as_mut_slice() {
                let date = NaiveDate::parse_from_str((&line).get(0).unwrap(), "%Y-%m-%d").unwrap();
                let time = NaiveTime::parse_from_str((&line).get(1).unwrap(), "%H:%M:%S%.f").unwrap();
                let mut vecs: Vec<i32> = Vec::new();
                for i in 2..57 {
                    vecs.push((&line).get(i).unwrap().parse::<i32>().unwrap());
                }
                res.push(Record {
                    date,
                    time,
                    recs: vecs,
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
                .map(|line| {
                    let date = NaiveDate::parse_from_str(line.get(0).unwrap(), "%Y-%m-%d").unwrap();
                    let time = NaiveTime::parse_from_str(line.get(1).unwrap(), "%H:%M:%S%.f").unwrap();
                    let mut recs: Vec<i32> = Vec::new();
                    for i in 2..57 {
                        recs.push(line.get(i).unwrap().parse::<i32>().unwrap());
                    }
                    Record {
                        date,
                        time,
                        recs,
                    }
                }).collect();
        }
    };
    m.entry(format!("{}, {}, big_map_rayon", lc, threads)).or_insert(rayon_now.elapsed().div_f64(rounds as f64));

    let par_now = Instant::now();
    println!("Running par");
    {
        // parallel map
        for i in 0..rounds {
            let _r: Vec<Record> = par_map_v1(
                &lines,
                &|_, line: &StringRecord| {
                    println!("{:?}", &line);
                    let date = NaiveDate::parse_from_str((&line).get(0).unwrap(), "%Y-%m-%d").unwrap();
                    let time = NaiveTime::parse_from_str((&line).get(1).unwrap(), "%H:%M:%S%.f").unwrap();
                    let mut vecs: Vec<i32> = Vec::new();
                    for i in 2..57 {
                        vecs.push((&line).get(i).unwrap().parse::<i32>().unwrap());
                    }
                    println!("Here");
                    Record {
                        date,
                        time,
                        recs: vecs,
                    }
                });
            println!("Finish round: {}",i );
        }
    };
    m.entry(format!("{}, {}, big_map_par", lc, threads)).or_insert(par_now.elapsed().div_f64(rounds as f64));
    m
}