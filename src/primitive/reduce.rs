pub fn reduce<T, U>(seq: &[T], f: &U) -> T
    where T: Copy,
          U: Fn(&T, &T) -> T
{
    let mut r: T = seq[0];
    for j in 1..seq.len() {
        r = f(&r, &seq[j]);
    }
    r
}
