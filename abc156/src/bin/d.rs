#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! mod_pow {
    ($ a : expr , $ n : expr , $ mod : expr ) => {{
        let mut ret = 1;
        let mut a = $a;
        let mut n = $n;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * a % $mod;
            }
            a = a * a % $mod;
            n >>= 1;
        }
        ret
    }};
}

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

const MOD: i64 = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: i64, a: i64, b: i64,
    };
    let all = mod_pow!(2, n, MOD) - 1;
    let ans = (all - mod_comb(n, a) + MOD) % MOD;
    let ans = (ans - mod_comb(n, b) + MOD) % MOD;
    echo!(ans);
}

fn mod_comb(n: i64, k: i64) -> i64 {
    let k = if k > n / 2 { n - k } else { k };
    let mut ret = 1;
    for k_i in 1..=k {
        ret *= (n - k_i + 1) * mod_inv!(k_i, MOD) % MOD;
        ret %= MOD;
    }
    ret
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
