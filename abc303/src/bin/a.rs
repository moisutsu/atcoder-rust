#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

fn similar_char(xy: (char, char)) -> bool {
    let (x, y) = xy;
    return x == y
        || x == '1' && y == 'l'
        || x == 'l' && y == '1'
        || x == 'o' && y == '0'
        || x == '0' && y == 'o';
}

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
        _n: i128,
        s: String,
        t: String,
    };
    let ans = s.chars().zip(t.chars()).all(similar_char);
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
