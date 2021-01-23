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
        As: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        let mut j = 0;
        let mut ans_i = 0;
        while j < N {
            let mut tmp_ans = 0;
            if As[j] >= As[i] {
                while j < N && As[j] >= As[i] {
                    tmp_ans += As[i];
                    j += 1;
                }
            }
            if tmp_ans > ans_i {
                ans_i = tmp_ans;
            }
            j += 1;
        }
        if ans_i > ans {
            ans = ans_i;
        }
    }
    echo!(ans);
}
