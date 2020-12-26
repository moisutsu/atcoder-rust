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
        S: Chars,
    };
    if S[S.len() - 1] == 's' {
        println!("{}es", S.iter().collect::<String>());
    } else {
        println!("{}s", S.iter().collect::<String>());
    }
}
