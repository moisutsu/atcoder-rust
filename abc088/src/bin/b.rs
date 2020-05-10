use proconio::*;

fn main() {
    input! {
        n: usize, mut a: [i32; n],
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut score = 0;
    for i in 0..n {
        score += if i % 2 == 0 { a[i] } else { -a[i] };
    }
    print!("{}", score);
}
