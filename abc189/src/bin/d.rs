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
        N: usize,
        Ss: [String; N],
    }
    let mut ans: u128 = 0;
    if Ss[0] == "AND" {
        ans += 1;
    } else {
        ans += 3;
    }
    for i in 1..N {
        if Ss[i] == "AND" {
        } else {
            ans += (2 as u128).pow(i as u32 + 1);
        }
    }
    echo!(ans);
}
