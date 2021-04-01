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
        _n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    };
    ab.sort_by_key(|&(a, b)| (b, Reverse(a)));
    let mut ans = 1;
    let mut cur = ab[0].1 - 1;
    for (a, b) in ab {
        if a > cur {
            ans += 1;
            cur = b - 1;
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
        )   *
        println!("{}" , s.join(" "));
    }
}
