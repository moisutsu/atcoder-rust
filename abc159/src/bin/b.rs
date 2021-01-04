#[allow(unused_imports)]
use proconio::marker::*;
use proconio::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($ x : expr ) => {
        println!("{}", $x);
    };
}

macro_rules! YesNo {
    ($ flag : expr ) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
}

fn is_kaibun(s: &[char]) -> bool {
    let mut retval = true;
    for (i, j) in (0..s.len()).zip((0..s.len()).rev()) {
        if s[i] != s[j] {
            retval = false;
            break;
        }
    }
    retval
}


#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }
    let ans = is_kaibun(&S)
        && is_kaibun(&S[0..(S.len() - 1) / 2])
        && is_kaibun(&S[(S.len() + 3) / 2 - 1..S.len()]);
    echo!(YesNo!(ans));
}
