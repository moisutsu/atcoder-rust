use proconio::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let a = N % 10;
    println!("{}",
        match a {
            2 | 4 | 5 | 7 | 9 => "hon",
            0 | 1 | 6 | 8 => "pon",
            3 => "bon",
            _ => "",
        }
    );
}
