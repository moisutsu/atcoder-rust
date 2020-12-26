use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

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
        r_1: i128, c_1: i128,
        r_2: i128, c_2: i128,
    }
    let mut ans = 0;
    let (mut a, mut b) = (r_1, c_1);
    loop {
        if a + b == r_2 + c_2 {
            ans += 1;
            break;
        } else if a - b == r_2 - c_2 {
            ans += 1;
            break;
        } else if (a - r_2).abs() + (b - c_2).abs() <= 3 {
            ans += 1;
            break;
        }

    }
    echo!(ans);
}
