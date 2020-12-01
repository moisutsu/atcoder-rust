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
        N: u128,
    }
    let modulo = 1000000007;
    echo!(if N == 1 {
        0
    } else {
        let mut ans = 0;
        ans += N * (N - 1);
        for _ in 0..N - 2 {
            ans *= 10;
            if ans > modulo {
                ans %= modulo;
            }
        }
        ans
    });
}
