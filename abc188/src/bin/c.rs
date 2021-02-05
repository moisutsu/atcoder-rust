#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::{max, min};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),*) => {
        let mut s = Vec::new();
        $(
            s.push(format!("{}" , $e));
        )*
        println!("{}" , s.join(" "));
    }
}

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
        N: u32,
        As: [i128; 2usize.pow(N)]
    };
    let mut players = As.clone();
    for p_i in (1..N as usize).rev() {
        let new_length = 2usize.pow(p_i as u32);
        let mut new_players = vec![0; new_length];
        for w_i in 0..new_length {
            new_players[w_i] = max(players[2 * w_i], players[2 * w_i + 1]);
        }
        players = new_players
    }
    let ans_rate = min(players[0], players[1]);
    echo!(As.get_index(&ans_rate).unwrap() + 1);
}
