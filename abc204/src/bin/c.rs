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
    let mut ans = 0;
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
    }
    for n_i in 0..n {
        ans += bfs(&g, n, n_i);
    }
    echo!(ans);
}

fn bfs(g: &[Vec<usize>], n: usize, s: usize) -> i64 {
    let mut cnt = 1;
    let mut q = VecDeque::new();
    q.push_front(s);
    let mut seen = vec![false; n];
    seen[s] = true;
    while let Some(from) = q.pop_back() {
        for &to in &g[from] {
            if seen[to] {
                continue;
            }
            seen[to] = true;
            cnt += 1;
            q.push_front(to);
        }
    }

    cnt
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
