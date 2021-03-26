#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize, mut Y: usize, A: usize, B: usize, C: usize,
        mut p: [i64; A],
        mut q: [i64; B],
        mut r: [i64; C],
    };
    let mut ans = 0;
    p.sort();
    q.sort();
    r.sort();
    while X != 0 || Y != 0 {
        let (mut p_last, mut q_last, mut r_last) = (-1, -1, -1);
        if let Some(&last) = p.last() {
            p_last = last;
        }
        if let Some(&last) = q.last() {
            q_last = last;
        }
        if let Some(&last) = r.last() {
            r_last = last;
        }
        if X == 0 {
            p_last = -1;
        }
        if Y == 0 {
            q_last = -1;
        }
        if p_last == max!(p_last, q_last, r_last) {
            X -= 1;
            ans += p.pop().unwrap();
        } else if q_last == max!(q_last, r_last) {
            Y -= 1;
            ans += q.pop().unwrap();
        } else if p_last <= q_last && X != 0 {
            X -= 1;
            ans += r.pop().unwrap();
        } else {
            Y -= 1;
            ans += r.pop().unwrap();
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
