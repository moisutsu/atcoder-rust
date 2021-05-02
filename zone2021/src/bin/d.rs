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
        s: Chars,
    };
    let mut t = VecDeque::new();
    let mut is_reversed = false;
    for c in s {
        if c == 'R' {
            is_reversed = !is_reversed;
        } else if is_reversed {
            if t.front() == Some(&c) {
                t.pop_front();
            } else {
                t.push_front(c);
            }
        } else if !is_reversed {
            if t.back() == Some(&c) {
                t.pop_back();
            } else {
                t.push_back(c);
            }
        }
    }
    if is_reversed {
        t = t.into_iter().rev().collect();
    }
    echo!(t.into_iter().collect::<String>());
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
