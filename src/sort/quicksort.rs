use rand::seq::SliceRandom;

use super::super::primitive::filter;
use rand::prelude::ThreadRng;

pub fn qsort(seq: &mut Vec<i32>, func: &dyn Fn(i32, i32) -> i32) -> Vec<i32> {
    return if seq.len() <= 1 {
        Vec::from(seq.clone())
    } else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &i32 = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let mut lt: Vec<i32> = filter(&mut seq.clone(), &|elt: i32| -> bool { func(elt, *p) < 0 });
        let mut eq: Vec<i32> = filter(&mut seq.clone(), &|elt: i32| -> bool { func(elt, *p) == 0 });
        let mut gt: Vec<i32> = filter(&mut seq.clone(), &|elt: i32| -> bool { func(elt, *p) > 0 });
        let mut left: Vec<i32> = qsort(&mut lt, func);
        let mut right: Vec<i32> = qsort(&mut gt, func);
        eq.extend(right);
        left.extend(eq);
        left
    }
}