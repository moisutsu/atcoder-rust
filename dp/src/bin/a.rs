#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
// use std::cmp::min;

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

macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        hs: [Isize1; N]
    };
    let mut dp = vec![std::isize::MAX; N];
    dp[0] = 0;
    dp[1] = (hs[1] - hs[0]).abs();
    for i in 2..N {
        // dp[i] = min(
        //     dp[i - 1] + (hs[i] - hs[i - 1]).abs(),
        //     dp[i - 2] + (hs[i] - hs[i - 2]).abs(),
        // );
        chmin!(
            dp[i],
            dp[i - 1] + (hs[i] - hs[i - 1]).abs(),
            dp[i - 2] + (hs[i] - hs[i - 2]).abs()
        );
    }
    echo!(dp[N - 1]);
}
