use rayon::prelude::*;

use crate::primitive::*;
use crate::constant::*;
use num_cpus::get;
use std::cmp::max;

#[allow(dead_code)]
pub fn non_inplace_par_quicksort<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let cut_size = max((3*seq.len()) / get(),  QS_THRESHOLD);
    non_inplace_par_quicksort_util(seq, func, cut_size)
}

#[allow(dead_code)]
fn non_inplace_par_quicksort_util<T, U>(seq: &Vec<T>, func: &U, cut_size: usize) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= cut_size {
        let mut ret = seq.clone(); // passing mutable instead
        ret.sort_unstable_by(|a, b| func(a, b).cmp(&0));
        ret
    }
    else {
        // let mut rng: ThreadRng = rand::thread_rng();
        // let p: &T = seq.choose(&mut rng).unwrap_or(&seq[seq.len() / 2]);
        let p: &T = &seq[seq.len()/2]; // try this first
        let ((lt, eq), gt) = rayon::join(
            || {
                rayon::join(
                    || par_filter(&seq, &|_i:usize, elt: &T| -> bool { func(elt, p) < 0 }),
                    || par_filter(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) == 0 })
                )
            },
            || par_filter(&seq, &|_i: usize, elt: &T| -> bool { func(elt, p) > 0 })
        );
        let (left , right) = rayon::join(
            || non_inplace_par_quicksort_util(&lt, func, cut_size),
            || non_inplace_par_quicksort_util(&gt, func, cut_size)
        );
        vec![left, eq, right].par_iter().flatten().map(|x| *x).collect() // try removing this line
    }
}


#[allow(dead_code)]
pub fn par_quicksort<T, U>(seq: &mut Vec<T>, func: &U)
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> i32
{
//    println!("inplace parallel quick sort of size {}", seq.len());
    let mut aux = vec_no_init(seq.len());
    let cut_size = max((2*seq.len()) / get(),  QS_THRESHOLD);
    par_quicksort_util(seq.as_mut_slice(), &mut aux, func, 0, cut_size)
}

pub fn par_quick_sort_slice<T, U>(seq: &mut [T], func: U)
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let mut aux = vec_no_init(seq.len());
    let cut_size = max((3*seq.len()) / get(),  QS_THRESHOLD);
    par_quicksort_util(seq, &mut aux, &func, 0, cut_size)
}

#[allow(dead_code, unused_variables)]
fn par_quicksort_util<T, U>(seq: &mut [T], aux: &mut [T], func: &U, passes: usize, cut_size: usize)
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if seq.len() <= cut_size {
        seq.sort_unstable_by(|a, b| func(a, b).cmp(&0));
        if passes % 2 == 1 {
            par_copy(aux, seq);
        }
    }
    else {
        let (lt_tot, eq_tot, mid_eq) = p_split3(seq, aux, func);
        let (seq_lt, seq_rest) = seq.split_at_mut(lt_tot);
        let (seq_eq, seq_gt) = seq_rest.split_at_mut(eq_tot);
        let (aux_lt, aux_rest) = aux.split_at_mut(lt_tot);
        let (aux_eq, aux_gt) = aux_rest.split_at_mut(eq_tot);
        rayon::join(
            || {
                if !mid_eq {
                    par_quicksort_util(aux_eq, seq_eq, func, passes + 1, cut_size)
                } else {
                    par_copy(seq_eq, &aux_eq);
                }
            },
            || {
                rayon::join(
                    || { par_quicksort_util(aux_lt, seq_lt, func, passes + 1, cut_size) },
                    || { par_quicksort_util(aux_gt, seq_gt, func, passes + 1, cut_size) }
                );
            }
        );
    }
}

#[allow(dead_code)]
pub fn rayon_par_quicksort<T, U>(seq: &mut Vec<T>, func: &U)
where T: Sync + Send + Copy ,
U: Sync + Send + Fn(&T, &T) -> i32
{
//    println!("rayon parallel quick sort of size {}", seq.len());
    seq.par_sort_unstable_by(|a,b| func(a,b).cmp(&0))
}

