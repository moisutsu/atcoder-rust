#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i64; N],
    };
    let mut ans = 1 << 60;
    for cnt in 0..N {
        for mut curs in (1..N).combinations(cnt) {
            curs.push(N);
            let mut v = vec![];
            let mut prev = 0;
            for cur in curs {
                let mut total = 0;
                for &a in &A[prev..cur] {
                    total |= a;
                }
                prev = cur;
                v.push(total);
            }
            let tmp = {
                let mut total = 0;
                for a in v {
                    total ^= a;
                }
                total
            };
            if tmp < ans {
                ans = tmp;
            }
        }
    }
    echo!(ans);
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}
