#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64, B: f64, C: f64,
    };

    const INF: f64 = (1i128 << 60) as f64;
    let (mut left, mut right) = (0.0, INF);
    while right - left > 0.000_000_000_000_1 {
        let mid = (left + right) / 2.0;
        let pos = f(mid, A, B, C);
        if pos >= 100.0 {
            right = mid;
        } else {
            left = mid;
        }
    }

    echo!(right);
}

#[allow(non_snake_case)]
fn f(t: f64, A: f64, B: f64, C: f64) -> f64 {
    A * t + B * (C * t * std::f64::consts::PI).sin()
}
