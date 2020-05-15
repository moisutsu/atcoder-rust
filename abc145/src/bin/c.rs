use proconio::*;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
    N: usize,
    xy: [(f64, f64); N],
    }
    let mut total_length = 0.0;
    for perm in (0..N).permutations(N) {
        let mut tmp_length = 0.0;
        for i in 0..N - 1 {
            tmp_length += ((xy[perm[i]].0 - xy[perm[i + 1]].0).powi(2) + (xy[perm[i]].1 - xy[perm[i + 1]].1).powi(2)).powf(0.5)
        }
        total_length += tmp_length;
    }
    println!("{}", total_length / (0..N).permutations(N).count() as f64);
}
