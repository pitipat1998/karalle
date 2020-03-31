use std::time::Duration;
use std::collections::HashMap;
use crate::benchmark::{run_quick_sort_benchmark,
                       run_merge_sort_benchmark,
                       run_sample_sort_benchmark};

#[allow(dead_code)]
pub fn run_sorting_benchmark(d: &String,
                             size: u64,
                             rounds: u128,
                             threads: usize)
    -> HashMap<String, Duration>
{
    let mut m : HashMap<String, Duration> = HashMap::new();
    println!("Running quick sort size: {:?}", size);
    let qs = run_quick_sort_benchmark(d, size, rounds, threads);
    println!("Running merge sort size: {:?}", size);
    let ms = run_merge_sort_benchmark(d, size, rounds, threads);
    println!("Running sample sort size: {:?}", size);
    let ss = run_sample_sort_benchmark(d, size, rounds, threads);
    m.extend(qs);
    m.extend(ms);
    m.extend(ss);
    m
}