pub fn map(seq: &mut Vec<usize>, func: &dyn Fn(usize) -> usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for item in seq {
        ret.push(func(*item))
    }
    Vec::from(ret)
}