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
        t: Chars
    };
    if t.len() > s.len() {
        echo!("UNRESTORABLE");
        return;
    }
    let mut anss = vec![];
    'outer: for s_i in 0..=s.len() - t.len() {
        let mut s = s.clone();
        for t_i in 0..t.len() {
            if s[s_i + t_i] != t[t_i] && s[s_i + t_i] != '?' {
                continue 'outer;
            }
        }
        for s_c in s[0..s_i].iter_mut() {
            if *s_c == '?' {
                *s_c = 'a';
            }
        }
        s[s_i..(t.len() + s_i)].clone_from_slice(&t[..]);
        for s_c in s.iter_mut().skip(s_i + t.len()) {
            if *s_c == '?' {
                *s_c = 'a';
            }
        }
        anss.push(s.into_iter().collect::<String>());
    }
    if anss.is_empty() {
        echo!("UNRESTORABLE");
    } else {
        echo!(anss.into_iter().min().unwrap());
    }
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
