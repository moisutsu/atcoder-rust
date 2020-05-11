use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, k: usize,
    }
    let anss: Vec<usize> = (1..a + b).filter(|&x| a % x == 0 && b % x == 0).collect();
    println!("{}", anss[anss.len() - k]);
}
