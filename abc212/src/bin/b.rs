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
        x: Chars,
    };
    let x = x
        .into_iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut is_weak = false;

    if x.clone().into_iter().collect::<HashSet<u32>>().len() == 1 {
        is_weak = true;
    }

    let mut x_1 = x[0];
    for (i, &x_i) in x.iter().enumerate().take(4).skip(1) {
        x_1 = (x_1 + 1) % 10;
        if x_i == x_1 {
            if i == 3 {
                is_weak = true;
            }
        } else {
            break;
        }
    }

    echo!(if is_weak { "Weak" } else { "Strong" });
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
