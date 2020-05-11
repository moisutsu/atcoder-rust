use proconio::*;

fn main() {
    input! {
        n: usize, y: usize,
    }
    let mut ans_flag = false;
    'outer: for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = n - i - j;
            if y == i * 10000 + j * 5000 + k * 1000 {
                print!("{} {} {}", i, j, k);
                ans_flag = true;
                break 'outer;
            }
        }
    }
    if !ans_flag {
        print!("-1 -1 -1");
    }
}
