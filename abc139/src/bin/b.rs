use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize, mut B: usize
    }
    let mut ans = 0;
    let mut power = 1;
    while power < B {
        power -= 1;
        power += A;
        ans += 1;
    }
    println!("{}", ans);
}
