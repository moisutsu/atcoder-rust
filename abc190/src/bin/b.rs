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
        N: usize, S: usize, D: usize,
        XYs: [(usize, usize); N],
    }
    let mut ans = false;
    for &(X, Y) in XYs.iter() {
        if X >= S || Y <= D {
            continue;
        }
        ans = true;
        break;
    }
    echo!(YesNo!(ans));
}
