#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: [i32; 5],
    }
    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i + 1);
        }
    }
}
