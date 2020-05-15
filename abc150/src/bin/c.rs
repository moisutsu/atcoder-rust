use itertools::Itertools;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [i32; N], Q: [i32; N],
    }
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let perm: Vec<Vec<i32>> = (1..=N)
        .permutations(N)
        .map(|v| v.iter().map(|&x| x as i32).collect())
        .collect();
    for i in 0..(0..N).permutations(N).count() {
        if P == perm[i] {
            a = i as i32;
        }
        if Q == perm[i] {
            b = i as i32;
        }
    }
    println!("{}", ((a - b)).abs());
}
