extern crate num_cpus;
extern crate rayon;

use self::rayon::prelude::*;
use crate::primitive::*;
use crate::constant::*;


// Version sqrt(n) splits
pub fn sqrt_splits_par_map<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = vec_no_init(seq.len());
    sqrt_splits_par_map_util(seq, &mut ret, func, 0, seq.len());
    ret
}

// Version n splits
pub fn n_splits_par_map<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = vec_no_init(seq.len());
    n_splits_par_map_utils(seq, &mut ret, func);
    ret
}

// Version half split
pub fn par_map<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send + Copy,
          U: Sync + Send + Copy,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    vec_init(seq.len(), &|i| func(i, &seq[i]), GRANULARITY)
}

// Version rayon
#[allow(dead_code)]
pub fn rayon_par_map<T, U, V>(seq: &Vec<T>, func: &V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    seq.par_iter().map(|x| func(1, x)).collect()
}


fn sqrt_splits_par_map_util<T, U, V>(
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
            let num_chunks: usize = ((n as f64) / (sqrt as f64)).ceil() as usize;

            rayon::scope(|s| {
                for (i, chunk) in ret.chunks_mut(sqrt).enumerate() {
                    if i < num_chunks - 1 {
                        s.spawn(move |_| {
                            sqrt_splits_par_map_util(
                                seq,
                                chunk,
                                func,
                                i * sqrt,
                                (i + 1) * sqrt,
                            );
                        });
                    } else {
                        let x = i * sqrt;
                        s.spawn(move |_| {
                            sqrt_splits_par_map_util(
                                seq,
                                chunk,
                                func,
                                x,
                                x + chunk.len(),
                            );
                        });
                    }
                }
            })
        }
    };
}

fn n_splits_par_map_utils<T, U, V>(seq: &[T], ret: &mut [U], func: &V)
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

