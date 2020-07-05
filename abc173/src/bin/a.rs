use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: i32,
    }
    if N % 1000 == 0 {
        println!("0");
    } else {
        println!("{}", 1000 - (N % 1000));
    }
}
