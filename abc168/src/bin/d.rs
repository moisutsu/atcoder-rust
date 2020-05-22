use proconio::*;
use proconio::marker::*;

use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, M: usize,
        mut AB: [(Usize1, Usize1); M],
    }
    let mut edges = vec![vec![]; N];
    for &(A, B) in AB.iter() {
        edges[A].push(B);
        edges[B].push(A);
    }
    let mut q = VecDeque::new();
    q.push_front(0);
    let mut numbers = vec![std::usize::MAX; N];
    while q.len() != 0 {
        let room = q.pop_back().unwrap();
        for &each in edges[room].iter() {
            if numbers[each] == std::usize::MAX {
                numbers[each] = room;
                q.push_front(each);
            }
        }
    }
    println!("Yes");
    for &number in numbers[1..].iter() {
        println!("{}", number + 1);
    }
}
