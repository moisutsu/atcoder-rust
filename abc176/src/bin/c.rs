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
        N: usize, A: [usize; N],
    }
    let mut ans = 0;
    let mut corrent_height = A[0];
    for &A_i in A.iter() {
        if A_i > corrent_height {
            corrent_height = A_i;
        }
        ans += corrent_height - A_i;
    }
    echo!(ans);
}
