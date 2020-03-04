fn scan_up<T, U>(seq: &[T], left: &mut [T], func: &U) -> T
    where T: Copy,
          U: Fn(&T, &T) -> T
{
    return if seq.len() <= 1 {
        seq[0]
    } else {
        let m: usize = seq.len() / 2;
        let l: T = scan_up(&seq[..m], &mut left[..m-1], func);
        let r: T = scan_up(&seq[m..], &mut left[m..], func);
        left[m-1] = l;
        func(&l, &r)
    }
}

fn scan_down<T, U>(right: &mut [T], left: &mut [T], func: &U, s: &T)
    where T: Copy,
          U: Fn(&T, &T) -> T
{
    if right.len() <= 1 {
        right[0] = *s;
    } else {
        let m: usize = right.len() / 2;
        let ns: T = func(s, &left[m-1]);
        scan_down(&mut right[..m], &mut left[0..m-1], func, s);
        scan_down(&mut right[m..], &mut left[m..], func, &ns);
    }
}

pub fn scan<T, U>(seq: &Vec<T>, func: U , s: &T) -> (Vec<T>, T)
    where T: Copy,
          U: Fn(&T, &T) -> T
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

