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

pub trait BinarySearch {
    type Element;
    fn lower_bound(&self, x: &Self::Element) -> usize;
    fn upper_bound(&self, x: &Self::Element) -> usize;
}
impl<T: Ord> BinarySearch for [T] {
    type Element = T;
    fn lower_bound(&self, x: &Self::Element) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less => {
                    low = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &Self::Element) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    low = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut As: [i128; N],
        mut Bs: [i128; N],
        mut Cs: [i128; N],
    };
    let mut ans = 0;
    As.sort_unstable();
    Bs.sort_unstable();
    Cs.sort_unstable();

    for j in 0..N {
        let i = As.lower_bound(&Bs[j]);
        let k = Cs.upper_bound(&Bs[j]);
        ans += i * (N - k);
    }
    echo!(ans);
}
