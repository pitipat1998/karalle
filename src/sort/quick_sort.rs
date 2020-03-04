use rand::seq::SliceRandom;

use super::super::primitive::filter;
use super::super::primitive::flatten;
use rand::prelude::ThreadRng;

fn quick_sort_utils<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Copy,
          U: Fn(&T, &T) -> i32
{
    return if seq.len() <= 1 {
       seq.to_vec()
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &T = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let lt: Vec<T> = filter(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 });
        let eq: Vec<T> = filter(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) == 0 });
        let gt: Vec<T> = filter(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) > 0 });
        flatten(vec![&quick_sort_utils(&lt, func), &eq, &quick_sort_utils(&gt, func)].as_ref())
    }
}

pub fn quick_sort<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Copy,
          U: Fn(&T, &T) -> i32
{
    quick_sort_utils(seq, &func)
}