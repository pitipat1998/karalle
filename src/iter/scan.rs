
fn scan_up(arr: &[usize], left: &[usize], f: &dyn Fn(usize) -> usize) -> usize {
    return if arr.len() == 1 {
        arr[0]
    } else {
        let m: usize = arr.len() / 2;
        let l: usize = scan_up(arr[])
        f()
    }
}