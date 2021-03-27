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
        N: usize,
        mut XC: [(i64, i64); N],
    };
    XC.sort_by_key(|&(X, C)| (C, X));
    let mut groups = vec![];
    let mut color = XC[0].1;
    let mut group = vec![];
    for (X, C) in XC {
        if C == color {
            group.push(X);
        } else {
            groups.push(group);
            group = vec![X];
            color = C;
        }
    }
    groups.push(group);

    let mut cand = vec![(0, 0)];
    for group in groups {
        let mut next_cand = vec![];
        for &(cost, x) in &cand {
            let (first, last) = (group[0], *group.last().unwrap());
            if x <= first && x <= last || x >= first && x >= last {
                if (x - first).abs() >= (x - last).abs() {
                    next_cand.push((cost + (x - first).abs(), first));
                } else {
                    next_cand.push((cost + (x - last).abs(), last));
                }
            } else {
                next_cand.push((cost + (x - last).abs(), last));
                next_cand.push((cost + (x - first).abs(), first));
            }
        }
        cand = next_cand;
    }
    echo!(cand
        .into_iter()
        .map(|(cost, cur)| cost + cur.abs())
        .min()
        .unwrap());
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
