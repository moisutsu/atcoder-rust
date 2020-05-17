use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut ans = 0;
    let mut m = 100;
    while X > m {
        ans += 1;
        m += ((m as f64) * 0.01) as usize;
    }
    println!("{}", ans);
}
