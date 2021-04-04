#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

const INF: usize = 1 << 60;

macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        h: usize, n: usize,
        ab: [(usize, usize); n]
    };
    let a_max = ab.iter().map(|&(a, _)| a).max().unwrap();
    let mut dp = vec![INF; h + a_max + 1];
    dp[0] = 0;
    for i in 0..n {
        for hp in 0..h + a_max {
            if dp[hp] == INF || hp + ab[i].0 > h + a_max {
                continue;
            }
            chmin!(dp[hp + ab[i].0], dp[hp] + ab[i].1);
        }
    }
    echo!(dp[h..h + a_max].iter().min().unwrap());
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
