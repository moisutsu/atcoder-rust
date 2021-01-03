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
        N: usize,
        A: [usize; N],
    }
    let mut gcd = 0;
    let mut ans = 0;
    for k in 2..=1000 {
        let mut tmp_gcd = 0;
        for &a in A.iter() {
            if a % k == 0 {
                tmp_gcd += 1;
            }
        }
        if tmp_gcd >= gcd {
            gcd = tmp_gcd;
            ans = k;
        }
    }
    echo!(ans);
}
