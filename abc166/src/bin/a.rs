use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}",
        if S == "ABC" { "ARC" } else { "ABC" }
    )
}
