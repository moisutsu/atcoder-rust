use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut flag = false;
    let mut ans = 1;
    let bound = 10u128.pow(18u32);
    input! {
        N: usize, A: [u128; N],
    }
    for a in A.iter() {
        ans *= a;
        if ans > bound {
            flag = true;
        }
        if *a == 0 {
            flag = false;
        }
    }
    if flag {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
