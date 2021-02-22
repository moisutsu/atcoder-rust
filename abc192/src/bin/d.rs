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
        X: Chars,
        M: i128,
    };
    let d = X.iter().max().unwrap().to_digit(10).unwrap() as i128;
    let X = X
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as i128)
        .collect::<Vec<i128>>();

    if X.len() == 1 {
        if X[0] <= M {
            echo!(1);
        } else {
            echo!(0);
        }
        std::process::exit(0);
    }
    let mut left = -1;
    let mut right = M + 1;
    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut y = 0;
        for x_i in 0..X.len() {
            y *= mid;
            y += X[x_i];
            if y > M {
                break;
            }
        }
        if y <= M {
            left = mid;
        } else {
            right = mid;
        }
    }
    echo!(left - d);
}
