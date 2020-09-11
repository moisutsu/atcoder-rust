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
        N: usize, X: usize, T: usize,
    }
    echo!(if N % X == 0 {
        N / X * T
    } else {
        (N / X + 1) * T
    })
}
