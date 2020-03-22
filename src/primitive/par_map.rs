extern crate num_cpus;
extern crate rayon;

use self::num_cpus::get;
use self::rayon::prelude::*;
use crate::primitive::*;
use crate::constant::*;
use std::cmp::min;


// Version sqrt(n) splits
pub fn par_map_v1<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = vec_no_init(seq.len());
    par_map_util_v1(seq, &mut ret, func, 0, seq.len());
    ret
}

// Version n splits
pub fn par_map_v2<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = vec_no_init(seq.len());
    par_map_util_v2(seq, &mut ret, func);
    ret
}

// Version half split
pub fn par_map_v3<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    vec_init(seq.len(), &|i, _| func(i, &seq[i]), GRANULARITY)
}

// Version rayon
#[allow(dead_code)]
pub fn par_map_v5<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    seq.par_iter().map(|x| func(1, x)).collect()
}


fn par_map_util_v1<T, U, V>(
    seq: &[T], ret: &mut [U],
    func: &V, s: usize, e: usize,
)
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let n = e - s;
    match n {
        _ if n <= GRANULARITY => {
            for i in s..e {
                ret[i - s] = func(i, &seq[i]);
            }
        }
        _ => {
            let sqrt: usize = (n as f64).sqrt().ceil() as usize;

            rayon::scope(|s| {
                for (i, chunk) in ret.chunks_mut(sqrt).enumerate() {
                    let start = i * sqrt;
                    let end = min(e, (i+1) * sqrt);
                        s.spawn(move |_| {
                            par_map_util_v1(
                                seq,
                                chunk,
                                func,
                                start,
                                end,
                            );
                        });
                }
            })
        }
    };
}

fn par_map_util_v2<T, U, V>(seq: &[T], ret: &mut [U], func: &V)
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    rayon::scope(|s| {
        for (i, chunk) in ret.chunks_mut(1).enumerate() {
            s.spawn(move |_| {
                chunk[0] = func(i, &seq[i]);
            });
        }
    })
}

