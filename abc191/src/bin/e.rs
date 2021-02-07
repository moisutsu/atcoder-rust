#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

use std::collections::VecDeque;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
        ABC: [(Usize1, Usize1, i128); M],
    };
    let mut g = vec![vec![-1; M]; M];
    for &(A, B, C) in ABC.iter() {
        if g[A][B] == -1 {
            g[A][B] = C;
        } else {
            g[A][B] = std::cmp::min(C, g[A][B]);
        }
    }

    for n_i in 0..N {
        let mut q = VecDeque::new();
        let mut times = vec![std::i128::MAX; N];
        let mut ans = std::i128::MAX;
        times[n_i] = 0;
        q.push_front(n_i);
        while q.len() != 0 {
            let from = q.pop_back().unwrap();
            for to in 0..g[from].len() {
                if g[from][to] == -1 {
                    continue;
                }
                if to == n_i {
                    ans = std::cmp::min(ans, g[from][to] + times[from]);
                }
                if g[from][to] + times[from] < times[to] {
                    times[to] = g[from][to] + times[from];
                    q.push_front(to);
                }
            }
        }
        if ans == std::i128::MAX {
            echo!(-1);
        } else {
            echo!(ans);
        }
    }
}
