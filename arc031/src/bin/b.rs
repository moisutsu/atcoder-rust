#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

macro_rules! YESNO {
    ($ flag : expr ) => {
        if $flag {
            "YES"
        } else {
            "NO"
        }
    };
}

const ADJ_4: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [Chars; 10],
    };
    let mut ans = false;

    'outer: for y in 0..10 {
        for x in 0..10 {
            let mut A = A.clone();
            A[y][x] = 'o';
            if dfs(&A) {
                ans = true;
                break 'outer;
            }
        }
    }

    echo!(YESNO!(ans));
}

fn dfs(A: &[Vec<char>]) -> bool {
    let mut cnt = 0;
    let mut stack = vec![];
    let mut seen = vec![vec![false; 10]; 10];
    for y in 0..10 {
        for x in 0..10 {
            if A[y][x] == 'x' {
                seen[y][x] = true;
            }
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            if seen[y][x] {
                continue;
            }
            cnt += 1;
            stack.push((x, y));
            seen[y][x] = true;
            while let Some((cx, cy)) = stack.pop() {
                for &(mx, my) in &ADJ_4 {
                    let sx = cx.wrapping_add(mx);
                    let sy = cy.wrapping_add(my);
                    if sx >= 10 || sy >= 10 || seen[sy][sx] {
                        continue;
                    }
                    stack.push((sx, sy));
                    seen[sy][sx] = true;
                }
            }
        }
    }
    cnt == 1
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
