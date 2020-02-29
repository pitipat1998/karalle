pub fn map(seq: &mut Vec<i32>, func: &dyn Fn(i32) -> i32) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for item in seq {
        ret.push(func(*item))
    }
    Vec::from(ret)
}