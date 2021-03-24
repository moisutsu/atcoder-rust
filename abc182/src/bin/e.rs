#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use proconio::{marker::*, *};
#[allow(unused_imports)]
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Block,
    Bulb,
    None,
    Light,
}

const ADJ_4: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize, W: usize, N: usize, M: usize,
        AB: [(Usize1, Usize1); N],
        CD: [(Usize1, Usize1); M],
    };
    let mut board = vec![vec![State::None; W]; H];
    for (C, D) in CD {
        board[C][D] = State::Block;
    }
    for (A, B) in AB {
        board[A][B] = State::Bulb;
        for &(mx, my) in &ADJ_4 {
            let (mut y, mut x) = (A, B);
            x = x.wrapping_add(mx);
            y = y.wrapping_add(my);
            while x < W && y < H && (board[y][x] == State::None || board[y][x] == State::Light) {
                board[y][x] = State::Light;
                x = x.wrapping_add(mx);
                y = y.wrapping_add(my);
            }
        }
    }
    echo!(board
        .into_iter()
        .map(|line| line
            .into_iter()
            .filter(|&state| state == State::Light || state == State::Bulb)
            .count())
        .sum::<usize>());
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
