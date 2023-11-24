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
        m: usize,
        a: [[Usize1; n]; m]
    };

    let mut v = vec![1; n * (n - 1) / 2 + 1];
    for (m_i, n_i) in iproduct!(0..m, 0..n - 1) {
        v[n * a[m_i][n_i] + a[m_i][n_i + 1]] = 0;
    }
    echo!(v.into_iter().sum::<i32>());
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
