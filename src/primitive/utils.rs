use rand::Rng;
use rand::prelude::ThreadRng;

const GRANULARITY: usize = 2048;

pub fn vec_no_init<T>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::with_capacity(n);
    unsafe { v.set_len(n) }
    v
}

fn vec_init_util<T, U>(v: &mut [T], s: usize, e: usize, f: &U)
    where T: Sync + Send,
          U: Sync + Send + Fn(usize, &mut ThreadRng) -> T,
{
    let n = e - s;
    if n <= GRANULARITY {
        let mut rng = rand::thread_rng();
        for i in s..e {
            let j = i - s;
            v[j] = f(i, &mut rng);
        }
    } else {
        let m = n / 2;
        let (l, r) = v.split_at_mut(m);
        rayon::join(
            || vec_init_util(l, s, s+m, f),
            || vec_init_util(r, s+m, e, f)
        );
    }
}

pub fn vec_init<T, U>(n: usize, f: &U) -> Vec<T>
    where T: Sync + Send,
          U: Sync + Send + Fn(usize, &mut ThreadRng) -> T,

{
    let mut v: Vec<T> = vec_no_init::<T>(n);
    vec_init_util(&mut v, 0, n, f);
    v
}

pub fn vec_zeroes(n: usize) -> Vec<usize>
{
    let v: Vec<usize> = vec_init(n, &|i, _| 0);
    v
}

fn par_copy_util<T: Sync + Send + Copy>(to: &mut [T], from: &[T], s: usize, e: usize) {
    let n = e - s;
    if n <= GRANULARITY {
        for i in s..e {
            let j = i - s;
            to[j] = from[i];
        }
    }
    else {
        let m = to.len()/2;
        let (to_l, to_r) = to.split_at_mut(m);
        rayon::join(
            || par_copy_util(to_l, from, s, s+m),
            || par_copy_util(to_r, from, s+m, e)
        );
    }
}

pub fn par_copy<T: Sync + Send + Copy>(to: &mut [T], from: &[T]) {
    if to.len() < from.len() {
        println!("par_copy: destination vector is less than source vector");
    }
    par_copy_util(to, from, 0, from.len());
}

