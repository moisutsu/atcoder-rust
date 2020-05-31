#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        hs: [i32; N],
    }
    let mut dp = vec![0; N];
    dp[1] = (hs[0] - hs[1]).abs();
    for i in 2..N {
        dp[i] = min(
            dp[i - 1] + (hs[i] - hs[i - 1]).abs(),
            dp[i - 2] + (hs[i] - hs[i - 2]).abs(),
        );
    }
    println!("{}", dp[N - 1]);
}
