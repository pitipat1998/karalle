pub fn map<T, U>(seq: &Vec<T>, func: &dyn Fn(usize, &T) -> U) -> Vec<U> {
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    for (i, item) in seq.iter().enumerate() {
        ret[i] = func(i, item);
    }
    ret
}