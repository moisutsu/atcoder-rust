#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

static MOD: i64 = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: Chars
    };

    let mut counts = vec![0; 8];

    for s_i in s.into_iter() {
        match s_i {
            'c' => {
                counts[0] += 1;
            }
            'h' => {
                counts[1] += counts[0];
                counts[1] %= MOD;
            }
            'o' => {
                counts[2] += counts[1];
                counts[2] %= MOD;
            }
            'k' => {
                counts[3] += counts[2];
                counts[3] %= MOD;
            }
            'u' => {
                counts[4] += counts[3];
                counts[4] %= MOD;
            }
            'd' => {
                counts[5] += counts[4];
                counts[5] %= MOD;
            }
            'a' => {
                counts[6] += counts[5];
                counts[6] %= MOD;
            }
            'i' => {
                counts[7] += counts[6];
                counts[7] %= MOD;
            }
            _ => (),
        }
    }

    echo!(counts[7]);
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
