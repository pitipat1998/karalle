use rand::seq::SliceRandom;

use super::super::primitive::filter;
use rand::prelude::ThreadRng;

pub fn qsort(seq: &mut Vec<i32>) -> Vec<i32> {
    if seq.len() <= 1 {
        return Vec::from(seq.clone());
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &i32 = seq.choose(&mut rng).unwrap_or(&seq[seq.len()/2]);
        let mut lt: Vec<i32>= filter(&mut seq.clone(), &|elt: i32| -> bool { elt < *p });
        let mut eq: Vec<i32> = filter(&mut seq.clone(), &|elt: i32| -> bool { elt == *p });
        let mut gt: Vec<i32> = filter(&mut seq.clone(), &|elt: i32| -> bool { elt > *p });
        let mut left: Vec<i32> = qsort(&mut lt);
        let mut right: Vec<i32> = qsort(&mut gt);
        eq.extend(right);
        left.extend(eq);
        return left
    }
}