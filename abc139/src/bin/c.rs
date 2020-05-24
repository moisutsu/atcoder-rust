use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    let mut ans = 0;
    let mut tmp_ans = 0;
    for i in 0..N - 1 {
        if H[i] >= H[i + 1] {
            tmp_ans += 1;
        } else {
            tmp_ans = 0;
        }
        if tmp_ans > ans {
            ans = tmp_ans;
        }
    }
    println!("{}", ans);
}
