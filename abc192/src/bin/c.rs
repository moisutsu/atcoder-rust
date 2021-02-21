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

trait StrToVecChar {
    fn to_vec_char(&self) -> Vec<char>;
}
impl StrToVecChar for str {
    fn to_vec_char(&self) -> Vec<char> {
        self.chars().collect::<Vec<char>>()
    }
}

fn g_1(x: usize) -> usize {
    let mut s = x.to_string().to_vec_char();
    s.sort_by_key(|&a| -(a as i32));
    s.iter().collect::<String>().parse().unwrap()
}

fn g_2(x: usize) -> usize {
    let mut s = x.to_string().to_vec_char();
    s.sort_by_key(|&a| a);
    s.iter().collect::<String>().parse().unwrap()
}

fn f(x: usize) -> usize {
    g_1(x) - g_2(x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    };
    let mut ans = N;
    for _ in 0..K {
        ans = f(ans);
    }
    echo!(ans);
}
