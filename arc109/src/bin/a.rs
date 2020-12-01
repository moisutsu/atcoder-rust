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
        a: usize, b: usize, x: usize, y: usize
    }
    let ans = if a == b {
        x
    } else if a > b {
        if 2 * x <= y {
            x + (a - b - 1) * 2 * x
        } else {
            x + (a - b - 1) * y
        }
    } else {
        if 2 * x <= y {
            x + (b - a) * 2 * x
        } else {
            x + (b - a) * y
        }
    };
    echo!(ans);
}
