fn scan_up(seq: &mut [usize], left: &mut [usize], func: &dyn Fn(usize, usize) -> usize) -> usize {
    return if seq.len() <= 1 {
        seq[0]
    } else {
        let m: usize = seq.len() / 2;
        let l: usize = scan_up(&mut seq[..m], &mut left[..m-1], func);
        let r: usize = scan_up(&mut seq[m..], &mut left[m..], func);
        left[m-1] = l;
        func(l, r)
    }
}

fn scan_down(right: &mut [usize], left: &mut [usize], func: &dyn Fn(usize, usize) -> usize, s: usize) {
    if right.len() <= 1 {
        right[0] = s;
    } else {
        let m: usize = right.len() / 2;
        let ns: usize = func(s, left[m-1]);
        scan_down(&mut right[..m], &mut left[0..m-1], func, s);
        scan_down(&mut right[m..], &mut left[m..], func, ns);
    }
}

pub fn scan(seq: &mut Vec<usize>, func: &dyn Fn(usize, usize) -> usize, s: usize) -> (Vec<usize>, usize) {
    let left: &mut [usize]  = &mut vec![0; seq.len()-1];
    let right: &mut [usize] = &mut vec![0; seq.len()];
    let total: usize = scan_up(seq, left, func);
    scan_down(right, left, func, s);
    (Vec::from(right), total)
}

