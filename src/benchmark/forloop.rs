use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::util::random_i16_list_generator;
use rand::Rng;
use std::cmp::min;

#[allow(dead_code)]
pub fn run_loop_benchmark(
    d: &String,
    size: u64,
    rounds: u128,
    threads: usize
) -> HashMap<String, Duration> {
    let mut res: HashMap<String, Duration> = HashMap::new();
    let key = format!("{}, {}, sort", &d, threads);
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        arr.sort_unstable();
        tot_time += t.elapsed();
    }
    res.entry(key).or_insert(tot_time.div_f64(rounds as f64));

    let key = format!("{}, {}, filter", &d, threads);
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        let _: Vec<&i16> = arr.iter().filter(|&x| x > &0).collect();
        tot_time += t.elapsed();
    }
    res.entry(key).or_insert(tot_time.div_f64(rounds as f64));

    let key = format!("{}, {}, flatten", &d, threads);
    let mut rng = rand::thread_rng();
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let mut c_size = size;
        let mut arr: Vec<Vec<i16>> = Vec::with_capacity(size as usize);
        while c_size > 0 {
            let s: u64 = if c_size == 1 { 1 } else {
                rng.gen_range(1, min(c_size, 1000001))
            };
            let tmp = random_i16_list_generator(s, -1000, 1001);
            arr.push(tmp);
            c_size -= s;
        }
        let now = Instant::now();
        let _: Vec<&i16> = arr.iter().flatten().collect();
        tot_time += now.elapsed()
    }
    res.entry(key).or_insert(tot_time.div_f64(rounds as f64));

    let key = format!("{}, {}, scan", &d, threads);
    let mut tot_time = Duration::new(0, 0);
    for _ in 0..rounds {
        let arr: Vec<i16> = random_i16_list_generator(size, -1000, 1001);
        let t = Instant::now();
        let _: Vec<i16> = arr.iter().scan(0, |state, &x|{
            *state = *state+x;
            Some(*state)
        }).collect();
        tot_time += t.elapsed();
    }
    res.entry(key).or_insert(tot_time.div_f64(rounds as f64));
    res
}