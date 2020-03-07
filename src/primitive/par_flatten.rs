use crate::primitive::par_map::par_map_v3;
use crate::primitive::par_scan::par_scan;

pub fn par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
    where T: Sync + Send + Copy
{
    let sizes: Vec<usize> = par_map_v3(seqs, |_i, &elt| -> usize { elt.len() });
    let (_x, tot): (Vec<usize>, usize) = par_scan(&sizes,
                                                  &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                                  &0);
    let mut ret = Vec::with_capacity(tot);
    unsafe { ret.set_len(tot) }
    // println!("_x:{:?}, tot:{}", _x, tot);
    par_flatten_util(seqs, &mut ret, &_x, tot);
    // TODO: parallelize this code
//    rayon::scope(|s| {
//        for i in 0..seqs.len() {
//            let off = x[i];
//            s.spawn(|_1| {
//                for j in 0..seqs[i].len() {
//                    s.spawn(|_2| {
//                        ret[off + j] = seqs[i][j];
//                    })
//                }
//            });
//        }
//    });
    ret
}

pub fn par_flatten_util<T: Clone + Sync + Send>(
    seq: &[&Vec<T>],
    ret: &mut [T],
    x: &[usize],
    tot: usize,
) {
    // println!("x:{:?}, tot:{}", x, tot);
    match tot {
        _ if tot <= 1 => {
            for i in 0..tot {
                let off = x[i];
                for j in 0..seq[i].len() {
                    ret[off + j] = seq[i][j].clone();
                }
            }
        }
        _ => {
            let half: usize = (seq.len() / 2) as usize;
            let (seq_l, seq_r) = seq.split_at(half);
            let (ret_l, ret_r) = ret.split_at_mut(half);
            let (x_l, x_r) = x.split_at(half);
            // println!("XL: {:?}, XR:{:?}", x_l, x_r);
            rayon::join(
                || { par_flatten_util(seq_l, ret_l, x_l, *x_l.last().unwrap_or(&0)) },
                || { par_flatten_util(seq_r, ret_r, x_r, tot - x_r[0]) },
            );
        }
    }
}