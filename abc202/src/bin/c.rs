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
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    };
    let mut a_count = HashMap::new();
    let mut c_count = HashMap::new();
    for a_i in a {
        *a_count.entry(a_i).or_insert(0) += 1;
    }
    for c_i in c {
        *c_count.entry(b[c_i]).or_insert(0) += 1;
    }
    let mut ans: i64 = 0;
    for a_key in a_count.keys() {
        if c_count.contains_key(a_key) {
            ans += a_count[a_key] * c_count[a_key];
        }
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
