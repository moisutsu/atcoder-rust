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
        n: i64,
        a: [usize; n],
        q: i64,
        bc: [(usize, usize); q],
    };
    let mut cnts = vec![0; 100_001];
    let mut ans = a.iter().sum::<usize>();
    for a_i in a {
        cnts[a_i] += 1;
    }
    for (b, c) in bc {
        ans += c * cnts[b];
        ans -= b * cnts[b];
        cnts[c] += cnts[b];
        cnts[b] = 0;
        echo!(ans);
    }
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
