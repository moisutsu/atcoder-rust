#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

const MOD: i64 = 1_000_000_007;

macro_rules! mod_inv {
    ($ a : expr , $ mod : expr ) => {{
        let (mut a, mut b, mut u, mut v) = ($a, $mod, 1, 0);
        while b != 0 {
            let t = a / b;
            a -= t * b;
            u -= t * v;
            b = std::mem::replace(&mut a, b);
            v = std::mem::replace(&mut u, v);
        }
        u %= $mod;
        if u < 0 {
            u += $mod;
        }
        u
    }};
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: i64,
    };
    let mut ans = 0;
    for size in 1..S / 3 + 1 {
        if S - size * 3 < 0 {
            break;
        }
        let remain = S - size * 3;
        let comb = f(remain + size - 1) * mod_inv!(f(remain), MOD) % MOD
            * mod_inv!(f(size - 1), MOD)
            % MOD;
        ans += comb;
        ans %= MOD;
    }
    echo!(ans);
}

fn f(n: i64) -> i64 {
    let mut total = 1;
    for n_i in 1..=n {
        total *= n_i;
        total %= MOD;
    }
    total
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
