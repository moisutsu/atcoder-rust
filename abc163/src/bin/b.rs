use proconio::*;
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
        A: [usize; M],
    }
    let ans: usize = A.iter().sum::<usize>();
    println!("{}",
        if N >= ans { (N - ans).to_string() } else { "-1".to_string() }
    )
}
