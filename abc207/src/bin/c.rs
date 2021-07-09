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
        tlr: [(i64, usize, usize); n]
    };
    let mut sections: Vec<Vec<usize>> = vec![];
    let mut ans = 0;
    for (t, mut l, mut r) in tlr {
        match t {
            1 => (),
            2 => r -= 1,
            3 => l += 1,
            4 => {
                l += 1;
                r -= 1;
            }
            _ => unreachable!(),
        };
        for section in &sections {
            if l <= section[1] && r >= section[0] {
                ans += 1;
            }
        }
        sections.push(vec![l, r]);
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
