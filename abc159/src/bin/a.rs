#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($ x : expr ) => {
        println!("{}", $x);
    };
}

fn c(n: usize, k: usize) -> usize {
    if n == 1 || n == 0 {
        0
    } else {
        let mut bunbo = 1;
        let mut bunshi = 1;
        for i in 1..=k {
            bunbo *= i;
        }
        for i in (n - k + 1)..=n {
            bunshi *= i;
        }
        bunshi / bunbo
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
    }
    let mut ans = 0;
    ans += c(N, 2) + c(M, 2);
    echo!(ans);
}
