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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let one_index = s.get_index(&'1').unwrap();
    echo!(if one_index % 2 == 0 {
        "Takahashi"
    } else {
        "Aoki"
    });
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
