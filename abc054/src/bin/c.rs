use proconio::*;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
        ab: [(usize, usize); M],
    }
    let ab: Vec<(usize, usize)> = ab.iter().map(|t| (t.0 - 1, t.1 - 1)).collect();
    let mut ans = 0;
    'outer: for perm in (0..N).permutations(N) {
        if perm[0] != 0 {
            continue;
        }
        for i in 0..perm.len() - 1 {
            if !ab.contains(&(perm[i], perm[i + 1])) && !ab.contains(&(perm[i + 1], perm[i])) {
                continue 'outer;
            }
        }
        ans += 1;
    }

    println!("{}", ans);
}
