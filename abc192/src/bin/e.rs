#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

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

use std::collections::BinaryHeap;

#[derive(Clone)]
struct Edge {
    to: usize,
    t: usize,
    k: usize,
}

const INF: usize = std::usize::MAX;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: Usize1,
        Y: Usize1,
        ABTK: [(Usize1, Usize1, usize, usize); M],
    };
    let mut dist = vec![INF; N];
    dist[X] = 0;
    let mut g = vec![vec![]; N];
    for &(A, B, T, K) in ABTK.iter() {
        g[A].push(Edge { to: B, t: T, k: K });
        g[B].push(Edge { to: A, t: T, k: K });
    }
    let mut q = BinaryHeap::new();
    q.push((0, X));
    while let Some((cost, pos)) = q.pop() {
        if pos == Y {
            break;
        }
        if cost > dist[pos] {
            continue;
        }
        for edge in &g[pos] {
            let next;
            if cost % edge.k == 0 {
                next = (cost + edge.t, edge.to);
            } else {
                next = (cost + edge.k - (cost % edge.k) + edge.t, edge.to)
            }
            if next.0 <= dist[next.1] {
                q.push(next);
                dist[next.1] = next.0;
            }
        }
    }

    echo!(if dist[Y] == INF { -1 } else { dist[Y] as i128 });
}
