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
        ab: [(usize, usize); M],
    };
    let mut ans = 0;
    for i in 0..M {
        let mut uf = UnionFind::new(N);
        for j in 0..M {
            if i == j {
                continue;
            }
            let (a, b) = ab[j];
            let (a, b) = (a - 1, b - 1);
            uf.union(a, b);
        }

        let mut count = 0;
        for j in 0..N {
            if uf.find(j) == j {
                count += 1;
            }
        }
        if count == 2 {
            ans += 1;
        }
    }

    echo!(ans);
}

struct UnionFind {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![None; n],
            sizes: vec![1; n],
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
        true
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
