#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, K: usize,
        mut p: [usize; N],
    }
    p.sort();
    println!("{}", (0..K).map(|x| p[x]).sum::<usize>());
}
