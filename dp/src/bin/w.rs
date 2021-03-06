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
        N: i32,
    }
    let mut ans = 0;
    ans = N;
    echo!(ans);
}
