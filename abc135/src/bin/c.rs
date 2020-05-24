use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N + 1],
        B: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if B[i] <= A[i] {
            ans += B[i];
        } else {
            ans += A[i];
            let next = B[i] - A[i];
            if next <= A[i + 1] {
                A[i + 1] -= next;
                ans += next;
            } else {
                ans += A[i + 1];
                A[i + 1] = 0;
            }
        }
    }
    println!("{}", ans);
}
