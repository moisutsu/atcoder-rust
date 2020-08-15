#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($ x : expr ) => {
        println!("{}", $x);
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: i128,
        P: [Usize1; N],
        C: [i128; N],
    }
    let mut ans = 0;
    let mut c_sum = vec![0; 2 * N + 1];
    let mut total = 0;
    for i in 0..N - 1 {
        total += C[P[i]];
        c_sum[i + 1] = total;
        c_sum[N + i + 1];
    }
    if c_sum[N] >= 0 {
        ans += K / N as i128 * c_sum[N];
        K -= K % N as i128;
    }
    let mut max = 0;
    for i in 0..2 * N - 1 {
        for j in i..2 * N {
            if i as i128 - j as i128 <= K {
                let score = c_sum[j] - c_sum[i];
                max = if score > max { score } else { max };
            }
        }
    }
    ans += max;
    echo!(ans);
}
