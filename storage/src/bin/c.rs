use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: i128, B: Chars
    }
    let B = format!("{}{}{}", B[0], B[2], B[3]);
    let B = B.parse().unwrap();
    println!("{}", A * B / 100);
}
