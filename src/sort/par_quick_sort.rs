use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::primitive::{par_filter_util_v2, vec_no_init};
use crate::primitive::par_filter_v1;
use crate::primitive::par_flatten;
use crate::primitive::par_map_v3;
use crate::primitive::par_scan;
use crate::primitive::par_copy;

const THRESHOLD: usize = 1000;

#[allow(dead_code)]
pub fn par_quick_sort<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    par_quick_sort_utils(seq, &func)
}

#[allow(dead_code)]
fn par_quick_sort_utils<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= THRESHOLD {
        let mut ret = seq.clone();
        ret.sort_unstable_by(|a, b| func(a, b).cmp(&0));
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
pub fn par_quick_sort_v2<T, U>(seq: &mut Vec<T>, func: U)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let mut aux = vec_no_init(seq.len());
    par_quick_sort_utils_v2(seq.as_mut_slice(), &mut aux, &func, 0)
}

#[allow(dead_code)]
fn par_quick_sort_utils_v2<T, U>(seq: &mut [T], aux: &mut [T], func: &U, passes: usize)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= THRESHOLD {
        seq.sort_by(|a, b| func(a, b).cmp(&0));
        if passes % 2 == 1 {
            par_copy(aux, seq);
        }
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let _length = seq.len();
        let p: &T = seq.choose(&mut rng).unwrap();

        let (aux_lt, aux_rest, lt_tot) =
        {
            let (lt_mapped, lt_x, lt_tot) = pref_sum(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 });
            let (aux_lt, aux_rest) = aux.split_at_mut(lt_tot);
            par_filter_util_v2(seq, aux_lt, &lt_mapped, &lt_x, 0);
            (aux_lt, aux_rest, lt_tot)
        };

        let (aux_eq, aux_gt, eq_tot) =
        {
            let (eq_mapped, eq_x, eq_tot) = pref_sum(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) == 0 });
            let (aux_eq, aux_gt) = aux_rest.split_at_mut(eq_tot);
            par_filter_util_v2(seq, aux_eq, &eq_mapped, &eq_x, 0);
            (aux_eq, aux_gt, eq_tot)
        };

        let aux_eq=
        {
            let (gt_mapped, gt_x, _gt_toto) = pref_sum(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) > 0 });
            par_filter_util_v2(seq, aux_gt, &gt_mapped, &gt_x, 0);
            aux_eq
        };


        let (seq_lt, seq_rest) = seq.split_at_mut(lt_tot);
        let (seq_eq, seq_gt) = seq_rest.split_at_mut(eq_tot);
        if passes % 2 == 0 {
            par_copy(seq_eq, aux_eq);
        }
        rayon::join(
            || { par_quick_sort_utils_v2(aux_lt, seq_lt, func, passes + 1) },
            || { par_quick_sort_utils_v2(aux_gt, seq_gt, func, passes + 1) }
        );
    }
}

fn pref_sum<T, U>(seq: &[T], func: &U) -> (Vec<usize>, Vec<usize>, usize)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<usize> = par_map_v3(&Vec::from(seq), &|i: usize, elt: &T| -> usize { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<usize>, usize) = par_scan(&mapped, &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 }, &0);
    (mapped, x, tot)
}

#[allow(dead_code)]
pub fn par_quick_sort_v3<T, U>(seq: &mut Vec<T>, func: U)
where T: Sync + Send + Copy,
U: Sync + Send + Fn(&T, &T) -> i32
{
    seq.par_sort_unstable_by(|a,b| func(a,b).cmp(&0))
}

