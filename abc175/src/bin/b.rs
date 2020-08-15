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
        N: usize,
        L: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                if L[i] != L[j]
                    && L[j] != L[k]
                    && L[k] != L[i]
                    && L[i] + L[j] > L[k]
                    && L[j] + L[k] > L[i]
                    && L[k] + L[i] > L[j]
                {
                    ans += 1;
                }
            }
        }
    }
    echo!(ans);
}
