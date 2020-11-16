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
        N: usize, A: [u128; N],
    }
    let mut ans = 0;
    let modulo = 1_000_000_007;
    let mut s = vec![0; N + 1];
    for i in 0..N {
        s[i + 1] = s[i] + A[i];
    }
    for i in 0..N - 1 {
        ans += A[i] * (s[N] - s[i + 1]);
        if ans >= modulo {
            ans %= modulo;
        }
    }
    echo!(ans);
}
