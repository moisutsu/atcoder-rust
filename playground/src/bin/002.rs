use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        A: usize, B: usize
    }
    let mut ans = false;
    for i in A..B + 1 {
        if i % K == 0 {
            ans = true;
        }
    }
    println!("{}",
        if ans { "OK" } else { "NG" }
    )
}
