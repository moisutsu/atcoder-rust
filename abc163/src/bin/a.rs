use proconio::*;
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: f64,
    }
    println!("{}", R * 2.0 * std::f64::consts::PI);
}
