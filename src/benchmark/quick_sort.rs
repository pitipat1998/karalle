//use std::collections::HashMap;
//use std::time::{Duration, Instant};
//
//use crate::primitive::*;
//use crate::sort::*;
//
//// par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
//#[allow(dead_code)]
//type QuicksortFunc<T> = dyn Sync + Send + Fn(&Vec<&Vec<T>>) -> Vec<T>;
//
//#[allow(dead_code)]
//fn benchmark_quick_sort<T: Copy + Sync, K>(vec: &Vec<T>, flat: K, rounds: u128) -> Duration
//    where K: Sync + Send + (Fn(&Vec<Vec<T>>) -> Vec<T>)
//{
//    let now = Instant::now();
//    for _ in 0..rounds {
//        let _ = flat(&vec);
//    }
//    now.elapsed().div_f32(rounds as f32)
//}
//
//#[allow(dead_code)]
//pub fn run_quick_sort_benchmark<T>(
//    d: &String,
//    v: &Vec<T>,
//    rounds: u128,
//    threads: usize
//) -> HashMap<String, Duration>
//where T: Copy + Sync + Send
//{
//    let mut result: HashMap<String, Duration> = HashMap::new();
//
//    let key = format!("{}, {}, par_quick_sort", &d, threads);
//    let duration = benchmark_quick_sort(&v, par_quick_sort, rounds);
//    result.entry(key).or_insert(duration);
//
//    let key = format!("{}, {}, par_quick_sort_v2", &d, threads);
//    let duration = benchmark_quick_sort(&v, par_quick_sort_v2, rounds);
//    result.entry(key).or_insert(duration);
//    result
//}