use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let mut ans = 0;
    for i in 1..K + 1 {
        for j in 1..K + 1 {
            for k in 1..K + 1 {
                ans += gcd(gcd(i, j), k);
            }
        }
    }
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }
    let (mut a, mut b) = (a, b);
    let mut r: usize;
    while a % b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    b
}
