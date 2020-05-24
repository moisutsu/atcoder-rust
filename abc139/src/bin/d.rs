use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: i128,
    }
    let ans: i128 = (N * (N - 1i128)) / 2i128;
    println!("{}", ans);
}
