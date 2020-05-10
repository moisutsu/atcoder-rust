use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        ds: [i32; n],
    }
    println!("{}",
        ds.into_iter().collect::<HashSet<i32>>().len()
    )
}
