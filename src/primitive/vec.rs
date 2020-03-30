use rand::prelude::ThreadRng;
use crate::constant::*;
use std::slice;
use crate::primitive::{single_sliced_for};

pub fn vec_no_init<T>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::with_capacity(n);
    unsafe { v.set_len(n) }
    v
}

fn vec_random_init_util<T, U>(v: &mut [T], s: usize, e: usize, f: &U, granularity: usize)
    where T: Sync + Send,
          U: Sync + Send + Fn(usize, &mut ThreadRng) -> T,
{
    let n = e - s;
    if n <= granularity {
        let mut rng = rand::thread_rng();
        for i in s..e {
            let j = i - s;
            v[j] = f(i, &mut rng);
        }
    } else {
        let m: usize = n / 2;
        let (l, r) = v.split_at_mut(m);
        rayon::join(
            || vec_random_init_util(l, s, s+m, f, granularity),
            || vec_random_init_util(r, s+m, e, f, granularity)
        );
    }
}

pub fn vec_random_init<T, U>(n: usize, f: &U, granularity: usize) -> Vec<T>
    where T: Sync + Send,
          U: Sync + Send + Fn(usize, &mut ThreadRng) -> T,
{
    let mut v: Vec<T> = vec_no_init::<T>(n);
    vec_random_init_util(&mut v, 0, n, f, granularity);
    v
}


pub fn vec_init<T, U>(n: usize, f: &U, _granularity: usize) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize) -> T,
{
    let mut v: Vec<T> = vec_no_init::<T>(n);
    single_sliced_for(&mut v, n, BLOCK_SIZE, &|seq, _i, s, e| {
        for j in s..e {
            seq[j] = f(j)
        }
    });
    v
}

pub fn vec_zeroes(n: usize) -> Vec<usize>
{
    let v: Vec<usize> = vec_init(n, &|_i| 0, GRANULARITY);
    v
}

pub fn no_split<T>(seq: &mut [T]) -> (&mut [T], &mut [T]) {
    let len = seq.len();
    let ptr: *mut T = seq.as_mut_ptr();
    unsafe {
        (slice::from_raw_parts_mut(ptr, len),
         slice::from_raw_parts_mut(ptr, len))
    }
}
