use proconio::*;

use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64, B: f64, H: f64, M: f64,
    }
    let A_k = 360.0 * (H + M / 60.0) / 12.0;
    let B_k = 360.0 * M / 60.0;
    let mut k = if A_k >= B_k { A_k - B_k } else { B_k - A_k };
    if k >= 180.0 {
        k = 360.0 - k;
    }
    k = k / 180.0 * PI;
    println!("{}",
        (A.powi(2) + B.powi(2) - 2.0 * A * B * k.cos()).powf(0.5)
    );
}
