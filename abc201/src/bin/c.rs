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
    let mut ans = 0;
    let mut includes = vec![];
    let mut not_includes = vec![];
    for (i, &s_i) in s.iter().enumerate() {
        match s_i {
            'o' => {
                includes.push(i);
            }
            'x' => {
                not_includes.push(i);
            }
            _ => (),
        }
    }
    'outer: for i in 0..10_000 {
        let mut s = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        for _ in 0..4 - s.len() {
            s.push(0);
        }
        for include in &includes {
            if !s.contains(include) {
                continue 'outer;
            }
        }
        for not_include in &not_includes {
            if s.contains(not_include) {
                continue 'outer;
            }
        }
        ans += 1;
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
