use rand::seq::SliceRandom;

use super::super::primitive::par_filter_v1;
use super::super::primitive::par_flatten;
use rand::prelude::ThreadRng;

const THRESHOLD: usize = 500;

#[allow(dead_code)]
fn par_quick_sort_utils<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= THRESHOLD {
        let mut ret = seq.clone();
        ret.sort_by(|a, b| func(a, b).cmp(&0));
        ret
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &T = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let lt: Vec<T> = par_filter_v1(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 });
        let eq: Vec<T> = par_filter_v1(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) == 0 });
        let gt: Vec<T> = par_filter_v1(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) > 0 });
        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        rayon::join(
            || { left = par_quick_sort_utils(&lt, func)},
                || { right = par_quick_sort_utils(&gt, func)}
        );
        par_flatten(&vec![left, eq, right])
    }
}

#[allow(dead_code)]
pub fn par_quick_sort<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    par_quick_sort_utils(seq, &func)
}

