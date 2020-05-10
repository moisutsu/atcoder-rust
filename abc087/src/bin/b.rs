use proconio::*;

fn main() {
    input! {
        a: usize, b: usize, c: usize, x: usize,
    }
    let mut count = 0;
    for a_i in 0..a + 1 {
        for b_i in 0..b + 1 {
            for c_i in 0..c + 1 {
                if x == a_i * 500 + b_i * 100 + c_i * 50 {
                    count += 1;
                }
            }
        }
    }
    print!("{}", count);
}
