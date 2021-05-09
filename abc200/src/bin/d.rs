#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

trait Joins {
    fn joins(&self, sep: &str) -> String;
}
impl<T: std::string::ToString + Copy> Joins for [T] {
    fn joins(&self, sep: &str) -> String {
        self.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let a = if n > 8 {
        a[..8].iter().cloned().collect_vec()
    } else {
        a
    };
    let mut ans = vec![vec![]; 201];

    for bit in 0..1 << a.len() {
        let mut indexes = vec![];
        for i in 0..a.len() {
            if bit & 1 << i == 0 {
                continue;
            }
            indexes.push(i + 1);
        }
        let ans_index = indexes.iter().map(|&index| a[index - 1]).sum::<usize>() % 200;
        if ans[ans_index].is_empty() {
            ans[ans_index] = indexes.clone();
        } else {
            echo!("Yes");
            echo!(ans[ans_index].len(), ans[ans_index].joins(" "));
            echo!(indexes.len(), indexes.joins(" "));
            return;
        }
    }
    echo!("No");
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
