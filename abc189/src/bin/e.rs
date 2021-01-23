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
        N: usize,
        XYs: [(i128, i128); N],
        M: usize,
        ops: [String; M],
        Q: usize,
        ABs: [(i128, i128); Q],
    }
    let mut ans = 0;
    ans = N;
    echo!(ans);
}
