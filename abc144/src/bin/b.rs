use proconio::*;

#[fastout]
fn main() {
    // 全探索
    // input! {
    //     n: usize,
    // }
    // let mut can = false;
    // for i in 1..10 {
    //     for j in 1..10 {
    //         if n == i * j {
    //             can = true;
    //         }
    //     }
    // }
    // println!("{}",
    //     if can { "Yes" } else { "No" }
    // )
    input! {
        n: usize
    }
    print!("{}",
        if (1..10).filter(|&x| n % x == 0 && n / x < 10).count() > 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
