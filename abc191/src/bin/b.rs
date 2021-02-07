use std::usize;

#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

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
        N: usize, X: usize,
        As: [usize; N],
    };
    echo!(As
        .into_iter()
        .filter(|&x| x != X)
        .collect::<Vec<usize>>()
        .joins(" "));
}
