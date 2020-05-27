use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: i32, B: i32, mut C: i32, D: i32
    }
    let mut i = 0;
    while A > 0 && C > 0 {
        if i % 2 == 0 {
            C -= B;
        } else {
            A -= D;
        }
        i += 1;
    }
    println!("{}",
        if i % 2 == 1 {
            "Yes"
        } else {
            "No"
        }
    )
}
