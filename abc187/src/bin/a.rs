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
        A: i32, B: i32
    };
    echo!(std::cmp::max(s(A), s(B)));
}

fn s(x: i32) -> i32 {
    let mut a = x;
    let mut total = 0;
    for _ in 0..3 {
        total += a % 10;
        a /= 10;
    }
    total
}
