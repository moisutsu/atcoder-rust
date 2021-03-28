#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut XC: [(i64, i64); N],
    };
    XC.sort_by_key(|&(X, C)| (C, X));
    let mut groups = vec![];
    let mut group = vec![];
    let mut prev = XC[0].1;
    for &(X, C) in &XC {
        if prev == C {
            group.push(X);
        } else {
            groups.push(group);
            group = vec![X];
            prev = C;
        }
    }
    groups.push(group);
    let mut dp = vec![vec![(1 << 60, 0); 2]; groups.len() + 1];
    dp[0][0] = (0, 0);
    dp[0][1] = (0, 0);
    for i in 0..groups.len() {
        let first = groups[i][0];
        let last = *groups[i].last().unwrap();

        chmin!(
            dp[i + 1][0],
            (
                dp[i][0].0 + (dp[i][0].1 - last).abs() + (first - last).abs(),
                first
            ),
            (
                dp[i][1].0 + (dp[i][1].1 - last).abs() + (first - last).abs(),
                first
            )
        );
        chmin!(
            dp[i + 1][1],
            (
                dp[i][0].0 + (dp[i][0].1 - first).abs() + (first - last).abs(),
                last
            ),
            (
                dp[i][1].0 + (dp[i][1].1 - first).abs() + (first - last).abs(),
                last
            )
        );
    }
    echo!(min(
        dp[groups.len()][0].0 + dp[groups.len()][0].1.abs(),
        dp[groups.len()][1].0 + dp[groups.len()][1].1.abs()
    ));
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
