use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut xys: Vec<Vec<(usize, usize)>> = vec![];
    for _ in 0..N {
        input! {
            A: usize,
            xy: [(usize, usize); A],
        }
        xys.push(xy);
    }
    let mut ans = 0;
    'outer: for bit in 0..1 << N {
        let mut v: Vec<usize> = vec![];
        for i in 0..N {
            if bit & (1 << i) != 0 {
                v.push(i);
            }
        }
        for i in v.iter() {
            for xy in xys[*i].iter() {
                if xy.1 == 1 && !v.contains(&(xy.0 - 1)) {
                    continue 'outer;
                }
                if xy.1 == 0 && v.contains(&(xy.0 - 1)) {
                    continue 'outer;
                }
            }
        }
        if ans < v.len() {
            ans = v.len();
        }
    }
    println!("{}", ans);
}
