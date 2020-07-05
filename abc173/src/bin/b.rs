use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;
    for s in S {
        if s == "AC" {
            ac += 1;
        } else if s == "WA" {
            wa += 1;
        } else if s == "TLE" {
            tle += 1;
        } else {
            re += 1;
        }
    }
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
