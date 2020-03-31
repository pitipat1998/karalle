use rayon::prelude::*;
use crate::primitive::{par_scan, vec_no_init, par_for};

#[allow(dead_code)]
pub fn par_flatten<T>(seqs: &Vec<Vec<T>>) -> Vec<T>
    where T: Sync + Send + Copy
{
    let mut sizes: Vec<usize> = seqs.par_iter().map(|elt| { elt.len() }).collect();
    let (x, m) = par_scan(&mut sizes, &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 }, &0);
    let mut ret: Vec<T> = vec_no_init(m);
    par_for(&mut ret, 0, seqs.len(), &|rr, i| {
        let off = x[i];
        par_for(rr, 0, seqs[i].len(), &|rrr, j| {
            rrr[off + j] = seqs[i][j];
        }, 2000)
    }, 2000);
    ret
}

// using rayon's par_iter
#[allow(dead_code)]
pub fn rayon_par_flatten<T>(seqs: &Vec<Vec<T>>) -> Vec<T>
    where T: Sync + Send + Copy
{
    (&seqs).into_par_iter().cloned().flatten().collect()
}


