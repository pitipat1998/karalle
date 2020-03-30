use crate::primitive::no_split;
use crate::constant::*;

fn par_transpose_util<T>(from: &[T], to: &mut [T], rs: usize, rc: usize, rl: usize, cs: usize, cc: usize, cl: usize)
    where T: Send + Sync + Copy
{
    if cc * rc < TRANS_THRESHOLD {
        for i in rs..(rs+rc) {
            for j in cs..(cs+cc) {
                to[j * cl + i] = from[i * rl + j];
            }
        }
    } else if cc > rc {
        let l1 = cc / 2;
        let l2 = cc - l1;
        let (to1, to2) = no_split(to);

        rayon::join(
            || par_transpose_util(from, to1, rs, rc, rl, cs, l1, cl),
            || par_transpose_util(from, to2, rs, rc, rl, cs + l1, l2, cl)
        );
    } else {
        let l1 = cc / 2;
        let l2 = rc - l1;
        let (to1, to2) = no_split(to);
        rayon::join(
            || par_transpose_util(from, to1, rs, l1, rl, cs, cc, cl),
            || par_transpose_util(from, to2, rs + l1, l2, rl, cs, cc, cl)
        );
    }
}

pub fn par_transpose<T>(from: &[T], to: &mut[T], rc: usize, cc: usize)
    where T: Send + Sync + Copy
{
    par_transpose_util(from , to, 0, rc, cc, 0, cc, rc);
}
