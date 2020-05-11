use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut min: i32 = n.to_string().len() as i32;
    for i in 1..(n as f32).powf(0.5) as usize + 1 {
        if n % i != 0 {
            continue;
        }
        let j = n / i;
        let t = f(i, j);
        if min > t {
            min = t;
        }
    }
    println!("{}", min);
}

fn f(a: usize, b: usize) -> i32 {
    let n = a.to_string().len();
    let m = b.to_string().len();
    if n >= m { n as i32 } else { m as i32 }
}
