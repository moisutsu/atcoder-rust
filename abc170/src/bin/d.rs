#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, mut A: [usize; N],
    }
    let mut dp = vec![true; N];
    A.sort_unstable();
}
