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

macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, W: usize,
        wv: [(usize, usize); N],
    };
    let mut dp = vec![vec![0; W + 1]; N + 1];
    for n_i in 0..N {
        for w_i in 0..=W {
            chmax!(dp[n_i + 1][w_i], dp[n_i][w_i]);
            if w_i >= wv[n_i].0 {
                chmax!(dp[n_i + 1][w_i], dp[n_i][w_i - wv[n_i].0] + wv[n_i].1);
            }
        }
    }
    echo!(dp[N][W]);
}
