use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        mut S: String,
    }
    if S.len() <= K {
        println!("{}", S);
    } else {
        S.truncate(K);
        println!("{}...", S);
    }
}
