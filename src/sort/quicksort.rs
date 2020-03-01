use rand::seq::SliceRandom;

use super::super::primitive::filter;
use rand::prelude::ThreadRng;

pub fn qsort<T: Copy>(seq: &mut Vec<T>, func: &dyn Fn(&T, &T) -> i32) -> Vec<T> {
    return if seq.len() <= 1 {
        Vec::clone(seq)
    } else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &T = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let mut lt: Vec<T> = filter(&mut seq.clone(), &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 });
        let mut eq: Vec<T> = filter(&mut seq.clone(), &|_i: usize, elt: &T| -> bool { func(elt, p) == 0 });
        let mut gt: Vec<T> = filter(&mut seq.clone(), &|_i: usize, elt: &T| -> bool { func(elt, p) > 0 });
        let mut left: Vec<T> = qsort(&mut lt, func);
        let right: Vec<T> = qsort(&mut gt, func);
        eq.extend(right);
        left.extend(eq);
        left
    }
}