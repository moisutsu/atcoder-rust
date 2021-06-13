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
        a: i64, b: i64, c: i64
    };
    let c = if c % 2 == 0 { 2 } else { 1 };
    let a_c = a.pow(c);
    let b_c = b.pow(c);
    echo!(if a_c == b_c {
        "="
    } else if a_c > b_c {
        ">"
    } else {
        "<"
    });
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
