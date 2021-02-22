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

macro_rules! YesNo {
    ($ flag : expr ) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
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
        H: usize, W: usize,
        c: [Chars; H],
    };
    let mut ans = false;
    let dx: [usize; 4] = [!0, 0, 1, 0];
    let dy: [usize; 4] = [0, 1, 0, !0];
    let mut seen = vec![vec![false; W]; H];
    let mut s = (0, 0);
    let mut g = (0, 0);
    for h_i in 0..H {
        if let Some(w_i) = c[h_i].get_index(&'s') {
            s = (w_i, h_i);
        }
        if let Some(w_i) = c[h_i].get_index(&'g') {
            g = (w_i, h_i);
        }
    }
    let mut stack = Vec::new();
    stack.push(s);
    'outer: while stack.len() != 0 {
        let p = stack.pop().unwrap();
        for d_i in 0..4 {
            let added_p = (p.0.wrapping_add(dx[d_i]), p.1.wrapping_add(dy[d_i]));
            if added_p.0 < W && added_p.1 < H {
                if seen[added_p.1][added_p.0] {
                    continue;
                }
                if added_p == g {
                    ans = true;
                    break 'outer;
                }
                if c[added_p.1][added_p.0] == '.' {
                    seen[added_p.1][added_p.0] = true;
                    stack.push(added_p);
                }
            }
        }
    }
    echo!(YesNo!(ans));
}
