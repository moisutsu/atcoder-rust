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
        n: usize, q: usize,
        a: [usize; n],
        k: [usize; q]
    };
    for k_i in k {
        let mut right = n;
        let mut left = 0;

        if k_i < a[0] {
            echo!(k_i);
            continue;
        }

        if k_i >= a[n - 1] {
            echo!(k_i + n);
            continue;
        }

        while right - left > 1 {
            let mid = (right + left) / 2;
            if a[mid] - 1 - mid >= k_i {
                right = mid;
            } else {
                left = mid;
            }
        }

        echo!(k_i + right);
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
