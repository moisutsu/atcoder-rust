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
        N: Chars,
    }
    let mut ans = -1;
    'outer: for k in 0..N.len() {
        for bit in 0..1 << N.len() {
            let mut v = Vec::new();
            for i in 0..N.len() {
                if bit & (1 << i) != 0 {
                    v.push(i);
                }
            }
            if N.len() - v.len() != k {
                continue;
            }
            let mut total = 0;
            for &i in v.iter() {
                total += N[i] as i32 - '0' as i32;
            }
            if total % 3 == 0 {
                ans = k as i32;
                break 'outer;
            }
        }
    }

    echo!(ans);
}
