use super::map;
use super::scan;

pub fn filter(seq: &mut Vec<i32>, func: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut mapped: Vec<i32> = map(seq, &|elt: i32| -> i32 { if func(elt) {1} else {0}});
    let (pref, tot): (Vec<i32>, i32) = scan(&mut mapped, &|elt1: i32, elt2: i32| -> i32 { elt1 + elt2 }, 0);
    let mut pref = pref;
    let mut ret: Vec<i32> = Vec::new();
    for (i, elt) in mapped.iter().enumerate() {
        if *elt == 1 {
            ret.push(seq[i])
        }
    }
    Vec::from(ret)
}