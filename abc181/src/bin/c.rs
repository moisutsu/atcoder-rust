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
        xy: [(i64, i64); n]
    };
    for (i, j, k) in iproduct!(0..n, 0..n, 0..n) {
        if i == j || j == k || k == i {
            continue;
        }
        let dx1 = xy[j].0 - xy[i].0;
        let dx2 = xy[k].0 - xy[j].0;
        let dy1 = xy[j].1 - xy[i].1;
        let dy2 = xy[k].1 - xy[j].1;
        if dx2 * dy1 == dx1 * dy2 {
            echo!("Yes");
            return;
        }
    }
    echo!("No");
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
