use rand::prelude::ThreadRng;
use crate::constant::*;

pub fn vec_no_init<T>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::with_capacity(n);
    unsafe { v.set_len(n) }
    v
}

fn vec_init_util<T, U>(v: &mut [T], s: usize, e: usize, f: &U, granularity: usize)
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
            || vec_init_util(l, s, s+m, f, granularity),
            || vec_init_util(r, s+m, e, f, granularity)
        );
    }
}

pub fn vec_init<T, U>(n: usize, f: &U, granularity: usize) -> Vec<T>
    where T: Sync + Send,
          U: Sync + Send + Fn(usize, &mut ThreadRng) -> T,

{
    let mut v: Vec<T> = vec_no_init::<T>(n);
    vec_init_util(&mut v, 0, n, f, granularity);
    v
}

pub fn vec_zeroes(n: usize) -> Vec<usize>
{
    let v: Vec<usize> = vec_init(n, &|_i, _| 0, GRANULARITY);
    v
}

