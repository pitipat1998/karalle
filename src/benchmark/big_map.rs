use std::fmt::*;

use chrono::{NaiveDate, NaiveTime};
use serde::export::Formatter;
use serde_json::from_str;

use crate::util::file_reader::read_lines;
use csv::StringRecord;

struct Record {
    date: NaiveDate,
    time: NaiveTime,
    recs: Vec<i32>,
}

impl Record {
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

#[allow(irrefutable_let_patterns)]
pub fn big_map_seq() {
    let filename = "DEBS2012-cleaned-v3.txt.small";
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(filename).unwrap();
    let mut lines: Vec<StringRecord> = Vec::new();
    for result in rdr.records() {
        match result {
            Ok(line) => {
                lines.push(line);
            }
            Err(e) => println!("{:?}",e)
        }
    }
    println!("DONE");
    // sequential
    // let lc = lines.by_ref().count();
    let mut res: Vec<Record> = Vec::with_capacity(lines.len());
    for line in lines {
        let date = NaiveDate::parse_from_str(line.get(0).unwrap(), "%Y-%m-%d").unwrap();
        let time = NaiveTime::parse_from_str(line.get(1).unwrap(), "%H:%M:%S%.f").unwrap();
        let mut vecs: Vec<i32> = Vec::new();
        for i in 2..57 {
            vecs.push(line.get(i).unwrap().parse::<i32>().unwrap());
        }
        res.push(Record {
            date,
            time,
            recs: vecs
        })
    }
    println!("{:?}", res);
}