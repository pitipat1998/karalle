fn scan_up(seq: &mut [i32], left: &mut [i32], func: &dyn Fn(i32, i32) -> i32) -> i32 {
    return if seq.len() <= 1 {
        seq[0]
    } else {
        let m: usize = seq.len() / 2;
        let l: i32 = scan_up(&mut seq[..m], &mut left[..m-1], func);
        let r: i32 = scan_up(&mut seq[m..], &mut left[m..], func);
        left[m-1] = l;
        func(l, r)
    }
}

fn scan_down(right: &mut [i32], left: &mut [i32], func: &dyn Fn(i32, i32) -> i32, s: i32) {
    if right.len() <= 1 {
        right[0] = s;
    } else {
        let m: usize = right.len() / 2;
        let ns: i32 = func(s, left[m-1]);
        scan_down(&mut right[..m], &mut left[0..m-1], func, s);
        scan_down(&mut right[m..], &mut left[m..], func, ns);
    }
}

pub fn scan(seq: &mut Vec<i32>, func: &dyn Fn(i32, i32) -> i32, s: i32) -> (Vec<i32>, i32) {
    let left: &mut [i32]  = &mut vec![0; seq.len()-1];
    let right: &mut [i32] = &mut vec![0; seq.len()];
    let total: i32 = scan_up(seq, left, func);
    scan_down(right, left, func, s);
    (Vec::from(right), total)
}

