use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut count = 0;
    loop {
        a = a.iter().filter(|&x| x % 2 == 0).map(|x| x / 2).collect();
        if n == a.len() {
            count += 1;
        } else {
            break;
        }
    }
    println!("{}", count);
}
