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

fn cf(x: f64, r: f64) -> (i128, i128) {
    ((x - r).ceil() as i128, (x + r).floor() as i128)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: f64, Y: f64, R: f64,
    };
    let mut ans = 0;
    let (start, end) = cf(X, R);
    for i in start..=end {
        let p = (R.powf(2.0) - (X - i as f64).powf(2.0)).sqrt();
        let (bottom, top) = cf(Y, p);
        for _ in bottom..=top {
            ans += 1;
        }
    }
    echo!(ans);
}
