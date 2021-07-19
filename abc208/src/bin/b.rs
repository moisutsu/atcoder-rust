#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut p: i64,
    };
    let mut ans = 0;
    for n in (1..=10).rev() {
        let coin = factorial(n);
        let cnt = min(100, p / coin);
        ans += cnt;
        p -= coin * cnt;
        if p == 0 {
            break;
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
