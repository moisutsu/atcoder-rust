use proconio::*;

fn main() {
    input! {
        a: usize, b: usize,

    }
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" })
}
