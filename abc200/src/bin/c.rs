#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

fn combination(n: i64, r: i64) -> i64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => combination(n, r - 1) * (n - r + 1) / r,
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut cnts = vec![0; 200];
    let mut ans = 0;
    for a_i in a {
        cnts[a_i % 200] += 1;
    }
    for cnt in cnts {
        if cnt < 2 {
            continue;
        }
        ans += combination(cnt, 2);
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
