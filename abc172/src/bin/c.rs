use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize, K: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut ans = 0;
    let mut a_i = 0;
    let mut b_i = 0;
    let mut count = 0;
    while a_i < N && count + A[a_i] <= K {
        count += A[a_i];
        a_i += 1;
    }
    while b_i < M && count + B[b_i] <= K {
        count += B[b_i];
        b_i += 1;
    }
    if ans < a_i + b_i {
        ans = a_i + b_i;
    }
    while a_i != 0 {
        a_i -= 1;
        count -= A[a_i];
        while b_i < M && count + B[b_i] <= K {
            count += B[b_i];
            b_i += 1;
        }
        if ans < a_i + b_i {
            ans = a_i + b_i;
        }
    }

    println!("{}", ans);
}
