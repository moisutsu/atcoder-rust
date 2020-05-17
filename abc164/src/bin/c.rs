use proconio::*;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    let mut set: HashSet<String> = HashSet::new();
    for s in S.iter() {
        set.insert(s.clone());
    }
    let ans = set.iter().count();
    println!("{}", ans);
}
