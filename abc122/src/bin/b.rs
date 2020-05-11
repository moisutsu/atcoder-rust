use proconio::*;

#[fastout]
fn main() {
    input! {
        s: String
    }
    let acgt = String::from("ACGT");
    let mut count: usize = 0;
    let mut max: usize = 0;
    for i in 0..s.len() {
        if acgt.contains(&s.chars().nth(i).unwrap().to_string()) {
            count += 1;
        } else {
            if count > max {
                max = count;
            }
            count = 0;
        }
    }
    if count > max {
        max = count;
    }
    println!("{}", max);
}
