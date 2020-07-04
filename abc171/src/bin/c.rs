#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: Usize1,
    }
    let mut ids = vec![];
    while N > 25 {
        ids.push(N % 26);
        N /= 26;
        N -= 1;
    }
    ids.push(N);
    let alphabets = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let mut ans = "".to_string();
    for id in ids {
        ans = format!("{}{}", alphabets[id], ans);
    }
    println!("{}", ans);
}
