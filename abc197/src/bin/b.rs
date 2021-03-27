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
        H: usize, W: usize, X: Usize1, Y: Usize1,
        S: [Chars; H],
    };
    let y = Y;
    let Y = X;
    let X = y;
    let mut ans = 1;
    for i in (0..X).rev() {
        if S[Y][i] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    for i in X + 1..W {
        if S[Y][i] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    for i in (0..Y).rev() {
        if S[i][X] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    for i in Y + 1..H {
        if S[i][X] == '.' {
            ans += 1;
        } else {
            break;
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
