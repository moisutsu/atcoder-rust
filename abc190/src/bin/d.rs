#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::collections::HashMap;

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
        N: i128,
    }
    let mut ans = 1;
    let mut map = HashMap::new();
    let mut n = N;
    while n % 2 == 0 {
        // (*map.entry(2).or_insert(0)) += 1;
        n /= 2;
    }
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            (*map.entry(f).or_insert(0)) += 1;
            n /= f;
        } else {
            f += 2;
        }
    }
    if n != 1 {
        (*map.entry(n).or_insert(0)) += 1;
    }
    for key in map.keys() {
        ans *= map[key] + 1;
    }
    eprintln!("{:?}", map);
    echo!(ans * 2);
}
