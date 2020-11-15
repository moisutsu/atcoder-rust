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

fn count_match_char_in_str(a: &str, b: &str) -> usize {
    let mut count = 0;
    let a_v = a.to_vec_char();
    let b_v = b.to_vec_char();
    for i in 0..a.len() {
        if a_v[i] == b_v[i] {
            count += 1;
        }
    }
    count
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String, T: String,
    }
    let mut max_count = 0;
    for i in 0..=S.len() - T.len() {
        let count = count_match_char_in_str(&S[i..i + T.len()], &T);
        max_count = if count > max_count { count } else { max_count }
    }
    echo!(T.len() - max_count);
}
