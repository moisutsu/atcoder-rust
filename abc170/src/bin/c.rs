#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: i32, N: usize,
        p: [i32; N],
    }
    let mut ans = 1000;
    if X < 1 || X > 100 {
        ans = X;
    } else {
        for i in 0..=101 {
            if p.contains(&i) {
                continue;
            }
            if (X - i).abs() < (X - ans).abs() {
                ans = i;
            }
        }
    }
    println!("{}", ans);
}
