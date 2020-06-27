use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }
    let mut ans = 0;
    for i in 0..S.len() {
        if S[i] != T[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
