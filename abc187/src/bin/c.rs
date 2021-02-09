#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

use std::collections::HashSet;

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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Ss: [Chars; N],
    };
    let mut ans = "satisfiable".to_string();
    let mut set: HashSet<String> = HashSet::new();
    for S in Ss.into_iter() {
        let not_S = if S[0] == '!' {
            S[1..].iter().collect::<String>()
        } else {
            format!("!{}", S.iter().collect::<String>())
        };
        if set.contains(&not_S) {
            ans = if S[0] == '!' {
                not_S
            } else {
                S.iter().collect::<String>()
            };
            break;
        } else {
            set.insert(S.iter().collect::<String>());
        }
    }
    echo!(ans);
}
