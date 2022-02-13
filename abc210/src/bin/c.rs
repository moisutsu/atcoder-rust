#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n]
    };

    let mut map = HashMap::new();

    for c_i in c.iter().take(k) {
        *map.entry(c_i).or_insert(0) += 1;
    }
    let mut ans = map.keys().len();

    for i in k..n {
        *map.entry(&c[i]).or_insert(0) += 1;
        *map.entry(&c[i - k]).or_insert(0) -= 1;
        if map.get(&c[i - k]).unwrap() == &0 {
            map.remove(&c[i - k]);
        }
        ans = max(ans, map.keys().len());
    }

    echo!(ans);
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}
