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
        xy: [(i32, i32); N],
    };
    let mut ans = 0;
    for i in 0..N - 1 {
        for j in i + 1..N {
            if xy[j].0 - xy[i].0 > 0
                && xy[i].0 - xy[j].0 <= xy[j].1 - xy[i].1
                && xy[j].1 - xy[i].1 <= xy[j].0 - xy[i].0
            {
                ans += 1;
            } else if xy[j].0 - xy[i].0 < 0
                && xy[i].0 - xy[j].0 >= xy[j].1 - xy[i].1
                && xy[j].1 - xy[i].1 >= xy[j].0 - xy[i].0
            {
                ans += 1;
            }
        }
    }
    echo!(ans);
}
