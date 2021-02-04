#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        ps: [usize; N],
    };
    let mut dp = vec![vec![false; 10000]; N + 1];
    for n_i in 0..=N {
        dp[n_i][0] = true;
    }
    for n_i in 0..N {
        for p_i in 0..10000 {
            if dp[n_i][p_i] && p_i + ps[n_i] < 10000 {
                dp[n_i + 1][p_i] = true;
                dp[n_i + 1][p_i + ps[n_i]] = true;
            }
        }
    }
    echo!(dp[N].iter().filter(|&&x| x).count());
}
