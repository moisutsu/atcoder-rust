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

macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, W: usize,
        wv: [(usize, usize); N],
    };
    let MAX_V = 100001;
    let mut dp = vec![vec![std::usize::MAX / 10; MAX_V]; N + 1];
    for n_i in 0..=N {
        dp[n_i][0] = 0;
    }
    for n_i in 0..N {
        for v_i in 0..MAX_V {
            chmin!(dp[n_i + 1][v_i], dp[n_i][v_i]);
            if v_i >= wv[n_i].1 {
                chmin!(dp[n_i + 1][v_i], dp[n_i][v_i - wv[n_i].1] + wv[n_i].0);
            }
        }
    }
    for v_i in (0..MAX_V).rev() {
        if dp[N][v_i] <= W {
            echo!(v_i);
            break;
        }
    }
}
