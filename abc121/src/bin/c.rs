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
        N: usize, mut M: i128,
        mut AB: [(i128, i128); N],
    };
    let mut ans = 0;
    AB.sort_by_key(|&(A, _)| A);
    for (A, B) in AB {
        if M > B {
            M -= B;
            ans += A * B;
        } else {
            ans += M * A;
            break;
        }
    }
    echo!(ans);
}
