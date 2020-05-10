use proconio::*;

fn main() {
    input! {
        n: usize, a: i32, b: i32
    }
    let mut total = 0;
    for i in 1..n + 1 {
        let sum: i32 = i.to_string().chars().map(|c| c as i32 - 48).sum();
        if a <= sum && sum <= b {
            total += i;
        }
    }
    println!("{}", total);
}
