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
        n: usize, d: f64, h: f64,
        dh: [(f64, f64); n],
    };
    let mut ans = 0.;
    for (d_i, h_i) in dh {
        ans = if ans < (d * h_i - d_i * h) / (d - d_i) {
            (d * h_i - d_i * h) / (d - d_i)
        } else {
            ans
        };
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
