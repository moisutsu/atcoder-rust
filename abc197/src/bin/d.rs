#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use num::Complex;

type Pt = Complex<f64>;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: f64,
        x0: f64, y0: f64,
        xn2: f64, yn2: f64,
    };
    let p0 = Pt::new(x0, y0);
    let pn2 = Pt::new(xn2, yn2);
    let c = (p0 + pn2) / Pt::new(2., 0.);
    let r = Pt::from_polar(&1.0, &(2. * std::f64::consts::PI / N));
    let p1 = (p0 - c) * r + c;
    echo!(p1.re, p1.im);
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
