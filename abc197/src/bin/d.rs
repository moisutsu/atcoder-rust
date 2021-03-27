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
        N: usize,
        x0: f64, y0: f64,
        xn2: f64, yn2: f64,
    };
    let (cx, cy) = ((x0 + xn2) / 2., (y0 + yn2) / 2.);
    let ang = 360.0 / (N as f64) / 180.0 * std::f64::consts::PI;
    let (mx0, my0) = (x0 - cx, y0 - cy);
    let mx1 = mx0 * ang.cos() - my0 * ang.sin();
    let my1 = mx0 * ang.sin() + my0 * ang.cos();
    echo!(mx1 + cx, my1 + cy);
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
