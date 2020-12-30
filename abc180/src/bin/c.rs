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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: u128,
    }
    let mut x = 1;
    let mut ys = Vec::new();
    while x * x < N {
        if N % x == 0 {
            echo!(x);
            ys.push(N / x);
        }
        x += 1;
    }
    if x * x == N {
        echo!(x);
    }
    for &y in ys.iter().rev() {
        echo!(y);
    }
}
