#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut AB: [(i128, i128); N],
    };
    let mut ans = 0;
    let mut aoki_total = AB.iter().map(|&(A, _)| A).sum::<i128>();
    let mut takahashi_total = 0;

    AB.sort_unstable_by_key(|&(A, B)| -(2 * A + B));

    for (A, B) in AB.into_iter() {
        aoki_total -= A;
        takahashi_total += A + B;
        ans += 1;
        if takahashi_total > aoki_total {
            break;
        }
    }
    echo!(ans);
}
