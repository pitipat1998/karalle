use crate::primitive::par_map::par_map_v3;
use crate::primitive::par_scan::par_scan;

const THRESHOLD: usize = 2;

pub fn par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
    where T: Sync + Send + Copy
{
    let sizes: Vec<usize> = par_map_v3(seqs, |_i, &elt| -> usize { elt.len() });
    let (_x, tot): (Vec<usize>, usize) = par_scan(&sizes,
                                                  &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                                  &0);
    let mut _x = Vec::from(&_x[1..]);
    _x.push(tot);
    let mut ret = Vec::with_capacity(tot);
    unsafe { ret.set_len(tot) }
    // println!("_x:{:?}, tot:{}", _x, tot);
    par_flatten_util(seqs, &mut ret, &_x, 0, tot);
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

pub fn par_flatten_util<T: Copy + Sync + Send>(
    seq: &[&Vec<T>],
    ret: &mut [T],
    x: &[usize],
    s: usize,
    e: usize
) {
     // println!("x:{:?}, ret:{:?}, seq:{}, s:{}, e:{}", x, ret.len(), seq.len(), s, e);
    if seq.len() <= THRESHOLD {
        if seq.len() > 0 {
            let mut n = 0;
            let mut r = s;
            for i in 0..seq.len() {
                let off = n;
                n = x[i] - r;
                r = x[i];
                // println!("x[i]:{}, seq[i]:{}", off, seq[i].len());
                for j in 0..seq[i].len() {
                    ret[off + j] = seq[i][j];
                }
            }
        }
    } else {
        let half: usize = (seq.len() / 2) as usize;
        let (seq_l, seq_r) = seq.split_at(half);

        let (x_l, x_r) = x.split_at(half);

        let l_size = *x_l.last().unwrap_or(&0) - s;
        let (ret_l, ret_r) = ret.split_at_mut(l_size);
        // println!("x_l:{:?}, x_r:{:?}, ret_l:{:?}, ret_r:{}, seq_l:{}, seq_r:{}", x_l, x_r, ret_l.len(), ret_r.len(), seq_l.len(), seq_r.len());
        rayon::join(
            || { par_flatten_util(seq_l, ret_l, x_l, s,  s+l_size) },
            || { par_flatten_util(seq_r, ret_r, x_r, s+l_size, e) },
        );
    }
}