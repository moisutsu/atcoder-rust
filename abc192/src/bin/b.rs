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

macro_rules! YesNo {
    ($ flag : expr ) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    };
    let mut ans = true;
    for i in 0..S.len() {
        if i % 2 == 0 {
            if S[i].is_ascii_uppercase() {
                ans = false;
                break
            }
        } else {
            if S[i].is_ascii_lowercase() {
                ans = false;
                break
            }
        }
    }
    echo!(YesNo!(ans));
}
