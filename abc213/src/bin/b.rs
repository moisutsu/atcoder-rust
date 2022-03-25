#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

trait GetIndex {
    type Element;
    fn get_index(&self, value: &Self::Element) -> Option<usize>;
}
impl<T: PartialEq> GetIndex for [T] {
    type Element = T;
    fn get_index(&self, value: &Self::Element) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
}

trait Sorted {
    type Element;
    fn sorted(&self) -> Vec<Self::Element>;
    fn sorted_by_key<F: FnMut(&Self::Element) -> K, K: Ord>(&self, f: F) -> Vec<Self::Element>;
}
impl<T> Sorted for [T]
where
    T: Clone + Ord,
{
    type Element = T;
    fn sorted(&self) -> Vec<Self::Element> {
        let mut v = <&[T]>::clone(&self).to_vec();
        v.sort();
        v
    }
    fn sorted_by_key<F: FnMut(&Self::Element) -> K, K: Ord>(&self, f: F) -> Vec<Self::Element> {
        let mut v = <&[T]>::clone(&self).to_vec();
        v.sort_by_key(f);
        v
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let sorted_a = a.sorted();

    let key = sorted_a[sorted_a.len() - 2];

    echo!(a.get_index(&key).unwrap() + 1);
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
