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
        X: i128,
        K: i128,
        D: i128,
    }
    if X.abs() >= K * D {
        echo!((X.abs() - K * D).abs());
    } else {
        let toutatu_count = X.abs() / D;
        let remain_K = K - toutatu_count;
        let anss = vec![
            (X.abs() - D * toutatu_count).abs(),
            (X.abs() - D * (toutatu_count + 1)).abs(),
        ];
        echo!(anss[(remain_K % 2) as usize]);
    }
}
