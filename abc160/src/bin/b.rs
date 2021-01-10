use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

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
        X: usize,
    }
    let big_count = X / 500;
    let remain_X = X % 500;
    let small_count = remain_X / 5;
    echo!(big_count * 1000 + small_count * 5);
}
