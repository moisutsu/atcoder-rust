#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use num::{bigint::ToBigInt, BigInt};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        l: i64
    };
    echo!(combination(l - 1, 11));
}

fn factorial(n: i64) -> BigInt {
    let mut total = 1.to_bigint().unwrap();
    for i in 2..=n {
        total *= i.to_bigint().unwrap();
    }
    total
}

fn combination(n: i64, r: i64) -> BigInt {
    factorial(n) / factorial(r) / factorial(n - r)
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
