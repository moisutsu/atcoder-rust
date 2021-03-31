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
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![false; n]; n];
    let mut ans = 0;
    for (a, b) in ab {
        g[a][b] = true;
        g[b][a] = true;
    }
    'outer: for vs in (0..n).permutations(n) {
        if vs[0] != 0 {
            continue;
        }
        for (&from, &to) in vs.iter().tuple_windows() {
            if !g[from][to] {
                continue 'outer;
            }
        }
        ans += 1;
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
