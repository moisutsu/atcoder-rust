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
        s: Chars,
        t: Chars,
    }
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for s_i in 1..=s.len() {
        for t_i in 1..=t.len() {
            if s[s_i - 1] == t[t_i - 1] {
                chmax!(dp[s_i][t_i], dp[s_i - 1][t_i - 1] + 1);
            } else {
                chmax!(dp[s_i][t_i], dp[s_i - 1][t_i], dp[s_i][t_i - 1]);
            }
        }
    }
    let mut o_s = "".to_string();
    let (mut o_i, mut s_i, mut t_i) = (dp[s.len()][t.len()], s.len(), t.len());
    while o_i > 0 {
        if s[s_i - 1] == t[t_i - 1] {
            o_s = format!("{}{}", s[s_i - 1], o_s);
            o_i -= 1;
            s_i -= 1;
            t_i -= 1;
        } else if dp[s_i][t_i] == dp[s_i - 1][t_i] {
            s_i -= 1;
        } else {
            t_i -= 1;
        }
    }
    echo!(o_s);
}
