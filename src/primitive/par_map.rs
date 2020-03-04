extern crate rayon;
const THRESHOLD: usize = 100;

fn par_map_recurse<T, U, V>(seq: &[T], ret: &mut [U], func: &V)
where T: Sync + Send,
      U: Sync + Send,
      V: Sync + Send + (Fn(usize, &T) -> U)
{
    if seq.len() <= THRESHOLD {
        for (i, item) in seq.iter().enumerate() {
            ret[i] = func(i, item);
        }
    } else {
        let half: usize = seq.len() / 2;
        let (seq_l, seq_r) = seq.split_at(half);
        let (ret_l, ret_r) = ret.split_at_mut(half);
        rayon::join(
                || par_map_recurse(
                    seq_l,
                    ret_l,
                    func
                ),
                || par_map_recurse(
                    seq_r,
                    ret_r,
                   func
                )
        );
    }
}

pub fn par_map<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_recurse(seq, &mut ret, &func);
    ret
}