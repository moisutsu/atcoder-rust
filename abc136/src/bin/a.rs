use proconio::*;

#[fastout]
fn main() {
    input! {
        a: i32, b: i32, c: i32
    }
    let s = c - (a - b);
    println!("{}", if s >= 0 { s } else { 0 });
}
