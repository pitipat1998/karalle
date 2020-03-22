use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::primitive::*;
use serde::export::fmt::{Display, Debug};
use crate::constant::*;
use num_cpus::get;
use std::cmp::max;
use rayon::current_num_threads;

#[allow(dead_code)]
pub fn par_quick_sort<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    println!("non-inplace parallel quick sort of size {}", seq.len());
    let cut_size = max((3*seq.len()) / get(),  QS_THRESHOLD);
    par_quick_sort_utils(seq, func, cut_size)
}

#[allow(dead_code)]
fn par_quick_sort_utils<T, U>(seq: &Vec<T>, func: &U, cut_size: usize) -> Vec<T>
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= cut_size {
        let mut ret = seq.clone();
        ret.sort_unstable_by(|a, b| func(a, b).cmp(&0));
        ret
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let p: &T = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let ((lt, eq), gt) = rayon::join(
            || {
                rayon::join(
                    || par_filter_v3(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 }),
                    || par_filter_v3(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) == 0 })
                )
            },
            || par_filter_v3(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) > 0 })
        );
        let (left , right) = rayon::join(
            || par_quick_sort_utils(&lt, func, cut_size),
                || par_quick_sort_utils(&gt, func, cut_size)
        );
        vec![left, eq, right].par_iter().flatten().map(|x| *x).collect()
    }
}


#[allow(dead_code)]
pub fn par_quick_sort_v2<T, U>(seq: &mut Vec<T>, func: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    println!("inplace parallel quick sort of size {}", seq.len());
    let mut aux = vec_no_init(seq.len());
    let cut_size = max((3*seq.len()) / get(),  QS_THRESHOLD);
    par_quick_sort_utils_v2(seq.as_mut_slice(), &mut aux, func, 0, cut_size)
//    par_quick_sort_utils_v2(seq.as_mut_slice(), &mut aux, func, 0, QS_THRESHOLD)
}

pub fn par_quick_sort_slice<T, U>(seq: &mut [T], func: U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let mut aux = vec_no_init(seq.len());
    let cut_size = max((3*seq.len()) / get(),  QS_THRESHOLD);
    par_quick_sort_utils_v2(seq, &mut aux, &func, 0, cut_size)
}

#[allow(dead_code)]
fn par_quick_sort_utils_v2<T, U>(seq: &mut [T], aux: &mut [T], func: &U, passes: usize, cut_size: usize)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= cut_size {
        seq.sort_unstable_by(|a, b| func(a, b).cmp(&0));
        if passes % 2 == 1 {
            par_copy(aux, seq);
        }
    }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let _length = seq.len();
        let p: &T = seq.choose(&mut rng).unwrap();

        let (lt_tot, eq_tot, mid_eq) = p_split3(seq, aux, func);

        let (seq_lt, seq_rest) = seq.split_at_mut(lt_tot);
        let (seq_eq, seq_gt) = seq_rest.split_at_mut(eq_tot);
        let (aux_lt, aux_rest) = aux.split_at_mut(lt_tot);
        let (aux_eq, aux_gt) = aux_rest.split_at_mut(eq_tot);
        rayon::join(
            || {
                if !mid_eq {
                    par_quick_sort_utils_v2(aux_eq, seq_eq, func, passes + 1, cut_size)
                } else {
                    par_copy(seq_eq, &aux_eq);
                }
            },
            || {
                rayon::join(
                    || { par_quick_sort_utils_v2(aux_lt, seq_lt, func, passes + 1, cut_size) },
                    || { par_quick_sort_utils_v2(aux_gt, seq_gt, func, passes + 1, cut_size) }
                );
            }
        );
    }
}

#[allow(dead_code)]
pub fn par_quick_sort_v3<T, U>(seq: &mut Vec<T>, func: &U)
where T: Sync + Send + Copy + Display + Debug,
U: Sync + Send + Fn(&T, &T) -> i32
{
    println!("rayon parallel quick sort of size {}", seq.len());
    seq.par_sort_unstable_by(|a,b| func(a,b).cmp(&0))
}

