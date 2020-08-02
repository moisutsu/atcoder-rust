#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: i32,
    }
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
