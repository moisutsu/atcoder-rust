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
        n: i64, m: usize,
        mut a: [i64; m],
    };
    let mut ans = 0;
    a.push(0);
    a.push(n + 1);
    a.sort();
    let mut spaces = vec![];
    for (left, right) in a.into_iter().tuple_windows() {
        let space = right - left;
        if space != 1 {
            spaces.push(space - 1);
        }
    }

    if spaces.is_empty() {
        echo!(ans);
        return;
    }

    spaces.sort();
    let k = spaces[0];

    for space in spaces {
        ans += (space + k - 1) / k;
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
