pub fn map<T, U>(seq: &mut Vec<T>, func: &dyn Fn(usize, &T) -> U) -> Vec<U> {
    let mut ret: Vec<U> = Vec::new();
    for (i, item) in seq.iter().enumerate() {
        ret.push(func(i, item));
    }
    ret
}