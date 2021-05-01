#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

trait ToBase<T> {
    fn to_base(&self, base: T) -> T;
}
impl<T> ToBase<T> for T
where
    T: Copy
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::cmp::PartialOrd
        + std::convert::From<bool>,
{
    fn to_base(&self, base: T) -> T {
        let zero = T::default();
        let one = true.into();
        let ten = (one + one + one) * (one + one + one) + one;
        let mut n = *self;
        let mut ret = zero;
        let mut digit = one;
        while n > zero {
            ret = ret + n % base * digit;
            n = n / base;
            digit = digit * ten;
        }
        ret
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut n: i64, k: i64,
    };
    let mut ans = 0;
    while n > 0 {
        n /= k;
        ans += 1;
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
