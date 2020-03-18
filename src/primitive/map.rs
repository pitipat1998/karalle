use crate::primitive::vec_no_init;

pub fn map<T, U>(seq: &Vec<T>, func: &dyn Fn(usize, &T) -> U) -> Vec<U> {
    let mut ret: Vec<U> = vec_no_init(seq.len());
    for (i, item) in seq.iter().enumerate() {
        ret[i] = func(i, item);
    }
    ret
}