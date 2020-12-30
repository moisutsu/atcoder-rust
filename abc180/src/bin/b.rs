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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xs: [i128; N]
    }
    echo!(xs.iter().map(|&x| x.abs()).sum::<i128>());
    echo!(xs
        .iter()
        .map(|&x| x.pow(2))
        .map(|x| x as f64)
        .sum::<f64>()
        .powf(0.5));
    echo!(xs.iter().map(|&x| x.abs()).max().unwrap());
}
