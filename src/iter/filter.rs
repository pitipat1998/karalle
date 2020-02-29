use super::map;
use super::scan;

pub fn filter(seq: &mut Vec<usize>, func: &dyn Fn(usize) -> bool) -> Vec<usize> {
    let mut mapped: Vec<usize> = map(seq, &|elt: usize| -> usize { if func(elt) {1} else {0}});
    let (pref, tot): (Vec<usize>, usize) = scan(&mut mapped, &|elt1: usize, elt2: usize| -> usize { elt1 + elt2 }, 0);
    let mut pref = pref;
    let mut ret: Vec<usize> = Vec::new();
    for (i, elt) in mapped.iter().enumerate() {
        if *elt == 1 {
            ret.push(seq[i])
        }
    }
    Vec::from(ret)
}