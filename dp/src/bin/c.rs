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
        N: usize,
        abc: [(i128, i128, i128); N],
    }
    let mut dp = vec![(0, 0, 0); N + 1];
    for i in 0..N {
        chmax!(dp[i + 1].0, dp[i].1 + abc[i].1, dp[i].2 + abc[i].2);
        chmax!(dp[i + 1].1, dp[i].0 + abc[i].0, dp[i].2 + abc[i].2);
        chmax!(dp[i + 1].2, dp[i].1 + abc[i].1, dp[i].0 + abc[i].0);
    }
    echo!(max!(dp[N].0, dp[N].1, dp[N].2));
}
