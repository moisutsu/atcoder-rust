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

#[allow(non_snake_case)]
fn YesNo(flag: bool) -> &'static str {
    if flag {
        "Yes"
    } else {
        "No"
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        As: [i32; N],
        Bs: [i32; N],
    }
    echo!(YesNo(
        As.into_iter()
            .zip(Bs.into_iter())
            .map(|(A, B)| A * B)
            .sum::<i32>()
            == 0
    ));
}
