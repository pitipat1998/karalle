extern crate rayon;

use num::PrimInt;
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;
use crate::primitive::{par_flatten, vec_no_init};

const THRESHOLD: usize = 100;

fn seq_sample_sort_util<T>(seq: &mut [T], k: usize, p: usize, start: usize, end: usize) -> Vec<T>
    where T: Copy + PrimInt + Sync + Send
{
    let n = end - start;
    if (n / k) < THRESHOLD {
        seq[start..end].sort_unstable();
        seq.to_vec()
    } else {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, seq.len());

        let mut result: Vec<Vec<T>> = Vec::with_capacity(n*(p + 2));
        for _ in 0..(p+2) {
            result.push(Vec::with_capacity(n));
        }
        let samp: &mut Vec<usize> = &mut (0..(p * k) as i32).map(|_| rng.sample(&range)).collect();
        samp.sort_unstable();

        let mut piv: Vec<T> = Vec::new();
        piv.push(T::min_value());
        for i in 1..(p - 1) {
            piv.push(num::cast::NumCast::from(samp[i * k]).unwrap());
        }
        piv.push(T::max_value());

        for &elm in seq.iter() {
            let jx: Vec<i32> = (1..piv.len() as i32)
                .into_par_iter()
                .filter(|&ij| piv[(ij - 1) as usize] < elm && elm <= piv[ij as usize])
                .collect();
            let j: usize = (*jx.first().unwrap()) as usize;
            result[j].push(elm.clone());
        }
        par_flatten(&result)
    }
}

#[allow(dead_code)]
pub fn seq_sample_sort<T>(seq: &mut [T], k: usize, p: usize) -> Vec<T>
    where T: Copy + PrimInt + Sync + Send
{
    seq_sample_sort_util(seq, k, p, 0, seq.len())
}