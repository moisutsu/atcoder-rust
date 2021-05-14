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
        n: i64,
    };
    let mut map = HashMap::new();
    let mut ans = 0;
    for n_i in 1..=n {
        let s = n_i.to_string().chars().collect::<Vec<char>>();
        let first = s[s.len() - 1];
        let last = s[0];
        *map.entry((first, last)).or_insert(0) += 1;
    }
    for &(first, last) in map.keys() {
        if map.contains_key(&(last, first)) {
            ans += map[&(first, last)] * map[&(last, first)];
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
