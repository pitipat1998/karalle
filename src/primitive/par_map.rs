extern crate num_cpus;
extern crate rayon;

use self::num_cpus::get;
use self::rayon::prelude::*;

const THRESHOLD: usize = 5;

fn par_map_utils<T, U, V>(
    seq: &[T], ret: &mut [U],
    func: &V, _s: usize, _e: usize,
)
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let n = _e - _s;
    if n <= THRESHOLD {
        for i in _s.._e {
            ret[i - _s] = func(i, &seq[i]);
        }
    } else {
        let sqrt: usize = (n as f64).sqrt().ceil() as usize;
        let num_chunks: usize = ((n as f64) / (sqrt as f64)).ceil() as usize;

        rayon::scope(|s| {
            for (i, chunk) in ret.chunks_mut(sqrt).enumerate() {
                if i < num_chunks - 1 {
                    s.spawn(move |_| {
                        par_map_utils(
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
                        par_map_utils(
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
}

pub fn par_map_v1<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_utils(seq, &mut ret, &func, 0, seq.len());
    ret
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

pub fn par_map_v2<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_util_v2(seq, &mut ret, &func);
    ret
}

pub fn par_map_v3<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    seq.par_iter().map(|x| func(1, x)).collect()
}

fn par_map_utils_v4<T, U, V>(
    seq: &[T], ret: &mut [U],
    func: &V, _s: usize, _e: usize,
    cpu: usize
)
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let n = _e - _s;
    if n <= THRESHOLD {
        for i in _s.._e {
            ret[i - _s] = func(i, &seq[i]);
        }
    } else {
        let size: usize = (n as f64 / (4 * cpu) as f64).ceil() as usize;

        rayon::scope(|s| {
            for (i, chunk) in ret.chunks_mut(size).enumerate() {
                if i < size - 1 {
                    s.spawn(move |_| {
                        par_map_utils_v4(
                            seq,
                            chunk,
                            func,
                            i * size,
                            (i + 1) * size,
                            cpu
                        );
                    });
                } else {
                    let x = i * size;
                    s.spawn(move |_| {
                        par_map_utils_v4(
                            seq,
                            chunk,
                            func,
                            x,
                            x + chunk.len(),
                            cpu
                        );
                    });
                }
            }
        })
    }
}

pub fn par_map_v4<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_utils_v4(seq, &mut ret, &func, 0, seq.len(), get());
    ret
}