pub fn map(seq: &mut Vec<usize>, func: &dyn Fn(usize) -> bool) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    for item in seq {
        ret.push(func(*item))
    }
    Vec::from(ret)
}