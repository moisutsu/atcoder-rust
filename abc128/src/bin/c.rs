use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
    }
    let mut s: Vec<Vec<usize>> = vec![vec![]; M];
    for i in 0..M {
        input! {
            k: usize,
            ss: [usize; k],
        }
        s[i] = ss;
    }
    input! {
        p: [usize; M],
    }
    let mut ans = 0;
    for bit in 0..1 << N {
        let mut counts = vec![0; M];
        for i in 0..N {
            if bit & (1 << i) != 0 {
                for j in 0..M {
                    if s[j].contains(&(i + 1)) {
                        counts[j] += 1;
                    }
                }
            }
        }
        if (0..M).filter(|&x| counts[x] % 2 == p[x]).count() == M {
            ans += 1;
        }
    }
    println!("{}", ans);
}
