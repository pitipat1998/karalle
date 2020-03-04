use crate::primitive::scan::scan;

pub fn flatten<T: Copy>(seqs: &Vec<&Vec<T>>) -> Vec<T> {
    let mut sizes: Vec<usize> = Vec::with_capacity(seqs.len());
    unsafe { sizes.set_len(seqs.len()) }
    for i in 0..seqs.len() {
        sizes[i] = seqs[i].len();
    }
    let (x, tot): (Vec<usize>, usize) = scan(&sizes,
                                       &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                       &0);
    let mut ret = Vec::with_capacity(tot);
    unsafe { ret.set_len(tot) }

    for i in 0..seqs.len() {
        let off = x[i];
        for j in 0..seqs[i].len() {
            ret[off + j] = seqs[i][j];
        }
    }
    ret

}