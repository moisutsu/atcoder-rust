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
        N: i32, X: usize,
        VP: [(usize, usize); N],
    }
    let X = X * 100;
    let mut total: usize = 0;
    let mut count = 0;
    for &(V, P) in VP.iter() {
        total += V * P;
        count += 1;
        if total > X {
            break;
        }
    }
    echo!(if total > X { count } else { -1 });
}
