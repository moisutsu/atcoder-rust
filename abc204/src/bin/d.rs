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
        mut t: [usize; n]
    };
    let t_sum = t.iter().sum::<usize>();
    let mut ans = if t_sum % 2 == 0 {
        t_sum / 2
    } else {
        t_sum / 2 + 1
    };

    loop {
        let mut set = HashSet::new();
        set.insert(0);
        for &t_i in &t {
            for &v in &set.clone() {
                set.insert(t_i + v);
            }
        }
        if set.contains(&ans) {
            break;
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
