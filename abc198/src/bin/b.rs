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
        n: Chars
    };
    let mut ans = false;
    let l = n.len();
    'outer: for i in 0..=l {
        let mut n_i = vec!['0'; i];
        for &c in n.iter() {
            n_i.push(c);
        }
        for (&c1, &c2) in n_i.iter().zip(n_i.iter().rev()) {
            if c1 != c2 {
                continue 'outer;
            }
        }
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
