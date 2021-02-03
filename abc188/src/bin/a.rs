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
        X: i32, Y: i32
    }
    echo!(YesNo!((X - Y).abs() < 3));
}
