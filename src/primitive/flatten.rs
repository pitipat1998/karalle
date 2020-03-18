use crate::primitive::scan::scan;
use crate::primitive::vec_no_init;

pub fn flatten<T: Copy>(seqs: &Vec<&Vec<T>>) -> Vec<T> {
    let mut sizes: Vec<usize> = vec_no_init(seqs.len());
    for i in 0..seqs.len() {
        sizes[i] = seqs[i].len();
    }
    let (x, tot): (Vec<usize>, usize) = scan(&sizes,
                                       &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                       &0);
    let mut ret = vec_no_init(tot);

    for i in 0..seqs.len() {
        let off = x[i];
        for j in 0..seqs[i].len() {
            ret[off + j] = seqs[i][j];
        }
    }
    ret

}