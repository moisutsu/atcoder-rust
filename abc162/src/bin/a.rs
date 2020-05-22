use proconio::*;
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
    }
    println!("{}",
        if N.iter().filter(|&c| *c == '7').count() >= 1 { "Yes" } else { "No" }
    )
}
