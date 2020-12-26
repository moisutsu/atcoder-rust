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
        a: i32, b: i32,
        c: i32, d: i32,
    }
    let ans = a * d - b * c;
    echo!(ans);
}
