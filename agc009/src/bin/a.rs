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
        AB: [(i128, i128); N],
    };
    let mut ans = 0;
    for i in (0..N).rev() {
        let A = AB[i].0 + ans;
        let amari = A % AB[i].1;
        if amari != 0 {
            ans += AB[i].1 - amari;
        }
    }
    echo!(ans);
}
