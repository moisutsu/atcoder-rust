#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

const MOD: i64 = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: i64, k: i64,
    };
    let mut ans = 0;
    for k_i in k..=n + 1 {
        let min_value = k_i * (k_i - 1) / 2;
        let max_value = k_i * (n - k_i + 1 + n) / 2;
        ans += max_value - min_value + 1;
        ans %= MOD;
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
