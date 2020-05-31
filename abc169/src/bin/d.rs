use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut v: Vec<usize> = vec![];
    let mut n = N;
    let mut count = 0;
    loop {
        let mut flag = true;
        for i in 2..((n as f64).powf(0.5) as usize) + 1 {
            if n % i == 0 {
                flag = false;
                v.push(1);
                n /= i;
                while n % i == 0 {
                    v[count] += 1;
                    n /= i;
                }
                count += 1;
            }
        }
        if flag {
            break;
        }
    }
    if v.is_empty() {
        v.push(1);
    }
    let mut count = 0;
    for &e in v.iter() {
        let mut i = 1;
        let mut e = e;
        while e >= i {
            count += 1;
            e -= i;
            i += 1;
        }
    }
    if N == 1 {
        count = 0;
    }
    println!("{}", count);
}
