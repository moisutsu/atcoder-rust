use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_unstable();
    let ans: usize = (1..N).map(|x| A[x]).sum();
    println!("{}", ans);
}
