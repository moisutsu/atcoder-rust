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
        n: usize, k: i64,
        t: [[i64; n]; n],
    };
    let mut ans = 0;

    for mut vs in (1..n).permutations(n - 1) {
        vs.insert(0, 0);
        vs.push(0);
        let mut time = 0;
        for (from, to) in vs.into_iter().tuple_windows() {
            time += t[from][to];
        }
        if time == k {
            ans += 1;
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
