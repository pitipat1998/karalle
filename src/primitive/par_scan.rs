fn scan_up<T, U>(seq: &[T], left: &mut [T], func: &U) -> T
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> T
{
    return if seq.len() <= 1 {
        seq[0]
    } else {
        let m: usize = seq.len() / 2;
        let mut l: T = seq[m];
        let mut r: T = seq[m];
        let (left_l, left_r) = left.split_at_mut(m);
        rayon::join(
            || { l = scan_up(&seq[..m], left_l, func); },
            || { r = scan_up(&seq[m..], left_r, func); }
        );
        left_l[m-1] = l;
        func(&l, &r)
    }
}

fn scan_down<T, U>(right: &mut [T], left: &mut [T], func: &U, s: &T)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> T
{
    if right.len() <= 1 {
        right[0] = *s;
    } else {
        let m: usize = right.len() / 2;
        let ns: T = func(s, &left[m-1]);
        let (right_l, right_r) = right.split_at_mut(m);
        let (left_l, left_r) = left.split_at_mut(m);
        rayon::join(
            || scan_down(right_l, left_l, func, s),
            || scan_down(right_r, left_r, func, &ns)
        );
    }
}

pub fn par_scan<T, U>(seq: &Vec<T>, func: U , s: &T) -> (Vec<T>, T)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let mut left: Vec<T>  = Vec::with_capacity(seq.len()-1);
    let mut right: Vec<T> = Vec::with_capacity( seq.len());
    unsafe {
        left.set_len(seq.len()-1);
        right.set_len(seq.len());
    }
    let total: T = scan_up(seq, &mut left, &func);
    scan_down(&mut right, &mut left, &func, s);
    (Vec::from(right), total)
}

