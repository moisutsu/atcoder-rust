#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, D: f64,
        XY: [[f64; 2]; N]
    }
    let mut ans = 0;
    for xy in XY {
        let x2 = xy[0].powf(2.0);
        let y2 = xy[1].powf(2.0);
        let d = (x2 + y2).powf(0.5);
        if d <= D {
            ans += 1
        }
    }
    println!("{}", ans);
}
