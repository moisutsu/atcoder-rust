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
        _N: usize, M: usize,
        ABs: [(Usize1, Usize1); M],
        K: usize,
        CDs: [(Usize1, Usize1); K],
    }
    let mut ans = 0;
    for a in 0..2usize.pow(K as u32) {
        let mut bit = a;
        let mut v = vec![];
        for i in 0..K {
            let (C, D) = CDs[i];
            if bit % 2 != 0 {
                v.push(C);
            } else {
                v.push(D);
            }
            bit /= 2;
        }
        let mut tmp_ans = 0;
        for &(A, B) in ABs.iter() {
            if v.contains(&A) && v.contains(&B) {
                tmp_ans += 1;
            }
        }
        if tmp_ans > ans {
            ans = tmp_ans;
        }
    }
    echo!(ans);
}
