#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, K: usize,
        hs: [i32; N],
    }
    let mut dp = vec![std::i32::MAX; N];
    dp[0] = 0;
    for i in 0..N {
        for j in 1..=K {
            if i + j >= N {
                break;
            }
            dp[i + j] = min(dp[i + j], dp[i] + (hs[i] - hs[i + j]).abs())
        }
    }
    println!("{}", dp[N - 1]);
}
