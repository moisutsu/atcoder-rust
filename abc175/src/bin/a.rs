#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($ x : expr ) => {
        println!("{}", $x);
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..3 {
        if S[i] == 'R' {
            tmp += 1;
        } else {
            ans = if tmp > ans { tmp } else { ans };
            tmp = 0;
        }
    }
    ans = if tmp > ans { tmp } else { ans };
    echo!(ans);
}
