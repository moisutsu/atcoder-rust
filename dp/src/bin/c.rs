#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        abc: [[usize; 3]; N],
    }
    let mut dp = vec![vec![0; 3]; N];
    for i in 0..=2 {
        dp[0][i] = abc[0][i];
    }
    for i in 1..N {
        for j in 0..=2 {
            dp[i][j] = min(
                dp[i - 1][(j + 1) % 3] + abc[i][j],
                dp[i - 1][(j + 2) % 3] + abc[i][j],
            )
        }
    }
    println!("{}", dp[N - 1].iter().max().unwrap());
}
