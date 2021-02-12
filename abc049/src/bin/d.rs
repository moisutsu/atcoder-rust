#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

use std::collections::HashMap;

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

trait Joins {
    fn joins(&self, sep: &str) -> String;
}
impl<T: std::string::ToString + Copy> Joins for [T] {
    fn joins(&self, sep: &str) -> String {
        self.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, K: usize, L: usize,
        pq: [(Usize1, Usize1); K],
        rs: [(Usize1, Usize1); L],
    };
    let mut pq_uf = UnionFind::new(N);
    let mut rs_uf = UnionFind::new(N);
    let mut map = HashMap::new();
    let mut ans = vec![0; N];

    for (p, q) in pq {
        pq_uf.union(p, q);
    }
    for (r, s) in rs {
        rs_uf.union(r, s);
    }

    for i in 0..N {
        *map.entry((pq_uf.find(i), rs_uf.find(i))).or_insert(0) += 1;
    }

    for i in 0..N {
        ans[i] = map[&(pq_uf.find(i), rs_uf.find(i))];
    }

    echo!(ans.joins(" "));
}

pub struct UnionFind {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
    groups_count: usize,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![None; n],
            sizes: vec![1; n],
            groups_count: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if let Some(parent) = self.parents[x] {
            let root_x = self.find(parent);
            self.parents[x] = Some(root_x);
            root_x
        } else {
            x
        }
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));
        if root_x == root_y {
            return false;
        }
        if self.sizes[root_x] >= self.sizes[root_y] {
            self.parents[root_y] = Some(root_x);
            self.sizes[root_x] += self.sizes[root_y];
        } else {
            self.parents[root_x] = Some(root_y);
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
