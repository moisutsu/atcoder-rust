use proconio::*;
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N - 1]
    }
    let mut anss: Vec<usize> = vec![0; N];
    for &a in A.iter() {
        anss[a] += 1;
    }
    for &ans in anss.iter() {
        println!("{}", ans);
    }
}
