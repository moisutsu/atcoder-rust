use proconio::*;
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let ans = (1..N + 1).filter(|&x| x % 5 != 0 && x % 3 != 0).sum::<usize>();
    println!("{}", ans);
}
