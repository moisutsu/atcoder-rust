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

macro_rules ! debug {($ ($ a : expr ) ,* $ (, ) * ) => {# [cfg (debug_assertions ) ] eprintln ! (concat ! ($ ("| " , stringify ! ($ a ) , "={:?} " ) ,*, "|" ) , $ (&$ a ) ,* ) ; } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xy: [(i128, i128); N],
    };
    let mut ans = false;

    'outer: for i in 0..N - 2 {
        for j in i + 1..N - 1 {
            for k in j + 1..N {
                let (xi, yi) = xy[i];
                let (xj, yj) = xy[j];
                let (xk, yk) = xy[k];
                if xk * (yj - yi) == yk * (xj - xi) {
                    ans = true;
                    debug!(xi, yi, xj, yj, xk, yk);
                    break 'outer;
                }
            }
        }
    }

    echo!(YesNo!(ans));
}
