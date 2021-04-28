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
        s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q],
    };
    let mut front = s[..n].to_vec();
    let mut back = s[n..].to_vec();
    for (t, a, b) in tab {
        match t {
            1 => {
                let (a, b) = (a - 1, b - 1);
                let ac = if a < n { front[a] } else { back[a - n] };
                let bc = if b < n { front[b] } else { back[b - n] };
                if a < n {
                    front[a] = bc;
                } else {
                    back[a - n] = bc;
                }
                if b < n {
                    front[b] = ac;
                } else {
                    back[b - n] = ac;
                }
            }
            2 => {
                std::mem::swap(&mut front, &mut back);
            }
            _ => unreachable!(),
        }
    }
    echo!(front.iter().chain(back.iter()).collect::<String>());
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
