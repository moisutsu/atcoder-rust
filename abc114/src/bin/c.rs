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

trait HaveNumber {
    fn have_number(&self, digit: i128) -> bool;
}

impl HaveNumber for i128 {
    fn have_number(&self, digit: i128) -> bool {
        let mut n = *self;
        while n != 0 {
            if n % 10 == digit {
                return true;
            }
            n /= 10;
        }
        false
    }
}

fn solve(N: i128, x: i128) -> i128 {
    if x > N {
        return 0;
    }
    let ret = solve(N, x * 10 + 3) + solve(N, x * 10 + 5) + solve(N, x * 10 + 7);
    if x.have_number(7) && x.have_number(5) && x.have_number(3) {
        ret + 1
    } else {
        ret
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: i128,
    };
    echo!(solve(N, 0));
}
