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
        HS: [(i128, i128); N],
    };

    const INF: i128 = 1 << 60;
    let (mut left, mut right) = (0, INF);
    while right - left > 1 {
        let mut ok = true;
        let mid = (left + right) / 2;

        let mut remain_times = vec![0; N];
        for i in 0..N {
            if HS[i].0 > mid {
                ok = false;
            }
            remain_times[i] = (mid - HS[i].0) / HS[i].1;
        }
        remain_times.sort_unstable();
        for i in 0..N {
            if remain_times[i] < i as i128 {
                ok = false;
                break;
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    echo!(right);
}
