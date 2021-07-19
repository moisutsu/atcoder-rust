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
        n: usize,
        k: usize,
        a: [usize; n]
    };
    let base_cnt = k / n;
    let remain = k - n * base_cnt;
    let mut sorted = a.clone();
    sorted.sort();
    let small_id = sorted[remain];
    for a_i in a {
        if a_i < small_id {
            echo!(base_cnt + 1);
        } else {
            echo!(base_cnt);
        }
    }
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
