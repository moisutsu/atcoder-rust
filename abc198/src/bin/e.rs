#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
const TRUE: &bool = &true;
const FALSE: &bool = &false;
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        cs: [usize; n],
        ab: [(Usize1, Usize1); n - 1]
    };
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = vec![0];
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    seen[0] = true;
    let root = vec![cs[0]];
    q.push_back((0, root));
    while let Some((from, prev_root)) = q.pop_front() {
        for &to in &g[from] {
            if seen[to] {
                continue;
            }
            seen[to] = true;
            if !prev_root.contains(&cs[to]) {
                ans.push(to);
                let mut cur_root = prev_root.clone();
                cur_root.push(cs[to]);
                q.push_back((to, cur_root));
            } else {
                q.push_back((to, prev_root.clone()));
            }
        }
    }
    ans.sort();
    for a in ans {
        echo!(a + 1);
    }
}

#[derive(Clone)]
pub struct BitSet {
    buf: Vec<u64>,
    size: usize,
}

impl BitSet {
    #[allow(dead_code)]
    pub fn new(size: usize) -> BitSet {
        BitSet {
            buf: vec![0; (size + 63) / 64],
            size,
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, i: usize, b: bool) {
        assert!(i < self.size);
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }

    #[allow(dead_code)]
    pub fn count_ones(&self) -> u32 {
        self.buf.iter().map(|x| x.count_ones()).sum()
    }

    #[allow(dead_code)]
    fn chomp(&mut self) {
        let r = self.size & 63;
        if r != 0 {
            if let Some(x) = self.buf.last_mut() {
                let d = 64 - r;
                *x = (*x << d) >> d;
            }
        }
    }
}

impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        [FALSE, TRUE][(self.buf[index >> 6] >> (index & 63)) as usize & 1]
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
