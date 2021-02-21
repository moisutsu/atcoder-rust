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
        M: u128,
    };
    let mut ans = 0;
    let d = *X.iter().max().unwrap() as u128 - '0' as u128;
    let 
    if X.len() == 1 {
        if X[0] <= M {

        }
        std::process::exit(0);
    }
    // for d_i in d + 1.. {
    //     let mut y = 0;
    //     for x_i in 0..X.len() {
    //         y *= d_i;
    //         y += X[x_i] as u128 - '0' as u128;
    //     }
    //     if y <= M {
    //         ans += 1;
    //     } else {
    //         break;
    //     }
    // }

    echo!(ans);
}
