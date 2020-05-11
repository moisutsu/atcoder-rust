use proconio::*;

#[fastout]
fn main() {
    input! {
        a: i32, b: i32, c: i32, mut x: i32, mut y: i32
    }
    let mut total = 0;
    let lower = if a + b >= c * 2 { c * 2 } else { a + b };
    let min = if x >= y { y } else { x };
    total += lower * min;
    if x - min == 0 {
        let least = y - min;
        if b <= c * 2 {
            total += least * b;
        } else {
            total += least * c * 2;
        }
    } else {
        let least = x - min;
        if a <= c * 2 {
            total += least * a;
        } else {
            total += least * c * 2;
        }
    }
    println!("{}", total);
}
