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
        ABs: [(usize, usize); M],
        K: usize,
        C: [usize; K],
    }
    let mut g = vec![vec![false; N]; N];
    for &(A, B) in ABs.iter() {
        g[A][B] = true;
    }
}
