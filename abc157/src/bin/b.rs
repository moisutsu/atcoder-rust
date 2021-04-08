#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! YesNo {
    ($ flag : expr ) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: [[i64; 3]; 3],
        n: usize,
        bs: [i64; n],
    };
    let mut ans = false;
    for b in bs {
        for (i, j) in iproduct!(0..3, 0..3) {
            if A[i][j] == b {
                A[i][j] = -1;
            }
        }
    }
    for i in 0..3 {
        if A[i].iter().sum::<i64>() == -3 {
            ans = true;
        }
        if A[0][i] + A[1][i] + A[2][i] == -3 {
            ans = true;
        }
    }
    if A[0][0] + A[1][1] + A[2][2] == -3 {
        ans = true;
    }
    if A[0][2] + A[1][1] + A[2][0] == -3 {
        ans = true;
    }
    echo!(YesNo!(ans));
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
