fn scan_up(arr: &mut [usize], left: &mut [usize], f: &dyn Fn(usize, usize) -> usize) -> usize {
    return if arr.len() <= 1 {
        arr[0]
    } else {
        let m: usize = arr.len() / 2;
        let l: usize = scan_up(&mut arr[..m], &mut left[..m-1], f);
        let r: usize = scan_up(&mut arr[m..], &mut left[m..], f);
        left[m-1] = l;
        f(l, r)
    }
}

fn scan_down(right: &mut [usize], left: &mut [usize], f: &dyn Fn(usize, usize) -> usize, s: usize) {
    if right.len() <= 1 {
        right[0] = s;
    } else {
        let m: usize = right.len() / 2;
        let ns: usize = f(s, left[m-1]);
        scan_down(&mut right[..m], &mut left[0..m-1], f, s);
        scan_down(&mut right[m..], &mut left[m..], f, ns);
    }
}

pub fn scan(arr: &mut Vec<usize>, f: &dyn Fn(usize, usize) -> usize, s: usize) -> (Vec<usize>, usize) {
    let left: &mut [usize]  = &mut vec![0; arr.len()-1];
    let right: &mut [usize] = &mut vec![0; arr.len()];
    let total: usize = scan_up(arr, left, f);
    scan_down(right, left, f, s);
    (Vec::from(right), total)
}

