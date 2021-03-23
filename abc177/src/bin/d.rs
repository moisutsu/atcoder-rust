#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    };
    let mut ans = 0;
    let mut uf = UnionFind::new(N);
    for (A, B) in AB {
        uf.union(A, B);
    }
    let mut groups = HashSet::new();
    for n_i in 0..N {
        groups.insert(uf.find(n_i));
    }
    for group in groups.into_iter() {
        ans = max(ans, uf.size(group));
    }
    echo!(ans);
}

pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    groups_count: usize,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            groups_count: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        let parent_x = self.parents[x];
        if parent_x == x {
            x
        } else {
            let root_x = self.find(parent_x);
            self.parents[x] = root_x;
            root_x
        }
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));
        if root_x == root_y {
            return false;
        }
        if self.sizes[root_x] >= self.sizes[root_y] {
            self.parents[root_y] = root_x;
            self.sizes[root_x] += self.sizes[root_y];
        } else {
            self.parents[root_x] = root_y;
            self.sizes[root_y] += self.sizes[root_x];
        }
        self.groups_count -= 1;
        true
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn groups_count(&self) -> usize {
        self.groups_count
    }
    pub fn size(&mut self, x: usize) -> usize {
        let root_x = self.find(x);
        self.sizes[root_x]
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
