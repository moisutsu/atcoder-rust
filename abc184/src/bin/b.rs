use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

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
        N: usize, X: usize,
        S: String,
    }
    let mut ans = X;
    for answer in S.chars() {
        if answer == 'o' {
            ans += 1;
        } else {
            if ans > 0 {
                ans -= 1;
            }
        }
    }
    echo!(ans);
}
