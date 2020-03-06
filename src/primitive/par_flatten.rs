use crate::primitive::par_scan::par_scan;
use crate::primitive::par_map::par_map_v3;

pub fn par_flatten<T>(seqs: &Vec<&Vec<T>>) -> Vec<T>
    where T: Sync + Send + Copy
{
    let sizes: Vec<usize> = par_map_v3(seqs, |_i, &elt| -> usize { elt.len() });
    let (_x, tot): (Vec<usize>, usize) = par_scan(&sizes,
                                       &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                       &0);
    let mut ret = Vec::with_capacity(tot);
    unsafe { ret.set_len(tot) }

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