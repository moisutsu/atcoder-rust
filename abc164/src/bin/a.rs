use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize, W: usize,
    }
    println!("{}",
        if W >= S {
            "unsafe"
        } else {
            "safe"
        }
    );
}
