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
        V: i32, T: i32, S: i32, D: i32
    };
    echo!(YesNo!(!(V * T <= D && D <= V * S)));
}
