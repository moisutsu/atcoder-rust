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
        n: usize, k: usize,
        p: [f64; n],
    };
    let mut ans = 0.;

    for &p_i in &p[0..k] {
        ans += (p_i + 1.) / 2.;
    }
    let mut crr = ans;
    for i in k..n {
        crr = crr - (p[i - k] + 1.) / 2. + (p[i] + 1.) / 2.;
        ans = ans.max(crr);
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
