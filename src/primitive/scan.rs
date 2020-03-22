pub fn scan<T, U>(seq: &[T], ret: &mut[T], f: &U, offset: &T) -> T
    where T: Copy,
          U: Fn(&T, &T) -> T
{
    let mut r = *offset;
    for i in 0..seq.len() {
        let t = seq[i];
        ret[i] = r;
        r = f(&r, &t);
    }
    r
}