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
        mut X: u128, Y: u128, A: u128, B: u128,
    }
    let mut ans = 0;
    while X * A <= X + B && X * A < Y {
        X *= A;
        ans += 1;
    }
    ans += (Y - 1 - X) / B;
    echo!(ans);
}
