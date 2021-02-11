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

macro_rules! YesNo {
    ($ flag : expr ) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut AB: [(i128, i128); N],
    };
    let mut ans = true;
    AB.sort_by_key(|&(_, B)| B);
    let mut total = 0;
    for &(A, B) in AB.iter() {
        total += A;
        if B < total {
            ans = false;
        }
    }

    echo!(YesNo!(ans));
}
