use proconio::*;

fn main() {
    input! {
        s: String,
    }
    println!("{}", s.chars().filter(|&x| x == '1').count())
}
