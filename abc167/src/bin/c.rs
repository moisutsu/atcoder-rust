use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize, X: usize,
        A: [[usize; M + 1]; N],
    }
    let mut ans = std::usize::MAX;
    for bit in 0..1 << N {
        let mut v: Vec<usize> = vec![0; M + 1];
        for i in 0..N {
            if bit & (1 << i) != 0 {
                for j in 0..M + 1 {
                    v[j] += A[i][j];
                }
            }
        }
        if (1..M + 1).filter(|&x| v[x] >= X).count() == M {
            ans = if ans > v[0] { v[0] } else { ans };
        }
    }
    if ans == std::usize::MAX {
        println!("{}", -1)
    } else {
        println!("{}", ans);
    }
}
