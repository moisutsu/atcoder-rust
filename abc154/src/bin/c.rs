#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! YESNO {
    ($ flag : expr ) => {
        if $flag {
            "YES"
        } else {
            "NO"
        }
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut set = HashSet::new();
    for a_i in a {
        set.insert(a_i);
    }
    echo!(YESNO!(set.len() == n));
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
