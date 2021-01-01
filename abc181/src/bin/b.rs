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
        AB: [(usize, usize); N],
    }
    let mut ans = 0;
    for &(A, B) in AB.iter() {
        ans += (A..=B).sum::<usize>();
    }
    echo!(ans);
}
