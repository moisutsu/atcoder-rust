#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize, Y: usize,
    }
    let mut ans = false;
    for i in 0..=X {
        if i * 2 + (X - i) * 4 == Y {
            ans = true;
            break;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
