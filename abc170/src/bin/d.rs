#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, mut A: [usize; N],
    }
    let mut dp = vec![true; N];
    A.sort_unstable();
    if N == 1 {
        println!("1");
    } else {
        for i in 0..N - 1 {
            if dp[i] {
                if A[i] == A[i + 1] {
                    dp[i] = false;
                }
                for j in i + 1..N {
                    if A[j] % A[i] == 0 {
                        dp[j] = false;
                    }
                }
            }
        }
        println!("{}", dp.iter().filter(|&&x| x).count())
    }
}
