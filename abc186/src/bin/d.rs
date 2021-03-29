#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! accumulate {
    ($ v : expr ) => {{
        let mut accumulate = vec![0];
        accumulate.extend(
            $v.iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                })
                .collect::<Vec<_>>(),
        );
        accumulate
    }};
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [i64; N]
    };
    let mut ans = 0;
    A.sort();
    let acc = accumulate!(A);
    for i in 0..N - 1 {
        ans += acc[N] - acc[i + 1] - A[i] * (N as i64 - i as i64 - 1);
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
