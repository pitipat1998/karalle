extern crate rayon;

use scoped_threadpool::Pool;

const THRESHOLD: usize = 100;

fn par_map_utils<T, U, V>(seq: &[T], ret: &mut [U], func: &V, s: usize, e: usize)
where T: Sync + Send,
      U: Sync + Send,
      V: Sync + Send + (Fn(usize, &T) -> U)
{
    let n = e - s;
    if n <= THRESHOLD {
        for i in s..e {
            ret[i-s] = func(i, &seq[i]);
        }
    } else {
        let sqrt: usize = (n as f64).sqrt().ceil() as usize;
        let num_chunks: usize = ((n as f64) / (sqrt as f64)).ceil() as usize;

        rayon::scope(|s| {
            for (i, chunk) in ret.chunks_mut(sqrt).enumerate() {
                if i < num_chunks-1 {
                        s.spawn(move |_| {
                            par_map_utils(
                                seq,
                                chunk,
                                func,
                                i * sqrt,
                                (i + 1) * sqrt
                            );
                        });
                } else {
                    s.spawn(move |_| {
                        par_map_utils(
                            seq,
                            chunk,
                            func,
                            i * sqrt,
                            seq.len()
                        );
                    });
                }
            }
        })
    }
}

pub fn par_map<'p, T, U, V>(seq: &Vec<T>, func: V) -> Vec<U>
    where T: Sync + Send,
          U: Sync + Send,
          V: Sync + Send + (Fn(usize, &T) -> U)
{
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_utils(seq, &mut ret, &func, 0, seq.len());
    ret
}