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
        n: usize, m: usize,
        ps: [(Usize1, String); m],
    };
    let mut acs = vec![false; n];
    let mut was = vec![0; n];

    for (p, s) in ps {
        if acs[p] {
            continue;
        }
        if s == "AC" {
            acs[p] = true;
        } else {
            was[p] += 1;
        }
    }
    echo!(
        (0..n).filter(|&i| acs[i]).count(),
        (0..n).filter(|&i| acs[i]).map(|i| was[i]).sum::<usize>()
    );
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
