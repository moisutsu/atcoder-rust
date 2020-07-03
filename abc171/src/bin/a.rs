#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: String,
    }
    println!("{}", if a == a.to_uppercase() { "A" } else { "a" });
}
