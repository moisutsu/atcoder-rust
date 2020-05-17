use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, K: usize,
    }
    let mut v = vec![0; N];
    for _ in 0..K {
        input! {
            d: usize,
            A: [usize; d],
        }
        for i in A.iter() {
            v[i - 1] = 1;
        }
    }
    let ans = v.iter().filter(|&&x| x == 0).count();
    println!("{}", ans);
}
