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
        D: [(usize, usize); N],
    };
    let mut count = 0;
    let mut ans = false;
    for (d_1, d_2) in D.iter() {
        if d_1 == d_2 {
            count += 1;
            if count == 3 {
                ans = true;
                break;
            }
        } else {
            count = 0;
        }
    }
    echo!(YesNo!(ans));
}
