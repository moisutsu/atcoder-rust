use proconio::*;
#[allow(unused_imports)]
use proconio::marker::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize, W: usize, K: usize,
        c: [Chars; H],
    }
    let mut ans = 0;
    for i_bit in 0..1 << H {
        for j_bit in 0..1 << W {
            let mut count = 0;
            for i in 0..H {
                for j in 0..W {
                    if (i_bit & (1 << i) != 0) && (j_bit & (1 << j) != 0) {
                        if c[i][j] == '#' {
                            count += 1;
                        }
                    }
                }
            }
            if count == K {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
