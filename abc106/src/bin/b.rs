use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut count = 0;
    for i in 1..n + 1 {
        if i % 2 != 0 && (1..i + 1).filter(|&x| i % x == 0).count() == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
