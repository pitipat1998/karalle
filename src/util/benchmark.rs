use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::util::read_csv;
use num::{PrimInt, Unsigned};
use std::iter::Product;
use std::str::FromStr;

mod benchmark {}

pub struct Benchmark<T, U>
    where T: Sync + Send,
          U: Sync + Send
{
    func: Box<dyn Fn(usize, &T) -> (Vec<U>)>,
    bench: Box<dyn Fn(&Vec<T>, T) -> (Vec<U>)>,
    func_name: String,
    version: String,
    files: Vec<String>,
}

impl<T,U> Benchmark<T,U>
    where T: Sync + Send,
          U: Sync + Send
{
    const NUM_LOOP: u32 = 1_000;
    pub(crate) fn run(&self) -> HashMap<String, Duration>
    {
        let mut ret: HashMap<String, Duration> = HashMap::new();
        // for f in self.files.iter() {
        //     let v: Vec<u128> = read_csv(f);
        //     let now = Instant::now();
        //     for _ in 0..Benchmark::NUM_LOOP {
        //         (self.bench)(&v, &self.func);
        //     }
        //     let du = now.elapsed() / Benchmark::NUM_LOOP;
        //     let key = format!("{}, {}, {}",
        //                       self.func_name, &f, self.version);
        //     ret.entry(key).or_insert(du);
        // }
        ret
    }
}