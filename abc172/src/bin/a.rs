use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
    }
    println!("{}", a + a.pow(2) + a.pow(3));
}
