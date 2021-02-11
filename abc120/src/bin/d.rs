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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
        AB: [(Usize1, Usize1); M],
    };
    let mut anss = vec![0; M];
    anss[M - 1] = N * (N - 1) / 2;
    let mut uf = UnionFind::new(N);
    for i in (1..M).rev() {
        let (A, B) = AB[i];
        let (size_A, size_B) = (uf.size(A), uf.size(B));
        if uf.union(A, B) {
            anss[i - 1] = anss[i] - size_A * size_B;
        } else {
            anss[i - 1] = anss[i];
        }
    }
    for ans in anss {
        echo!(ans);
    }
}

pub struct UnionFind {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
    groups_count: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![None; n],
            sizes: vec![1; n],
            groups_count: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if let Some(parent) = self.parents[x] {
            let root_x = self.find(parent);
            self.parents[x] = Some(root_x);
            root_x
        } else {
            x
        }
    }
    fn union(&mut self, x: usize, y: usize) -> bool {
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
    fn size(&mut self, x: usize) -> usize {
        let root_x = self.find(x);
        self.sizes[root_x]
    }
}
