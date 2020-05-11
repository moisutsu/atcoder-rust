use proconio::*;

fn main() {
    input! {
        n: usize,
        mut txys: [(i32, i32, i32); n],
    }
    txys.insert(0, (0, 0, 0));
    let mut can_go = true;
    for i in 1..n + 1 {
        let dt: i32 = txys[i].0 - txys[i - 1].0;
        let dist = (txys[i].1 - txys[i - 1].1).abs() + (txys[i].2 - txys[i - 1].2).abs();
        if dist > dt {
            can_go = false;
            break;
        }
        if (dt % 2) != (dist % 2) {
            can_go = false;
            break;
        }
    }
    println!("{}",
        if can_go { "Yes" } else { "No" }
    );
}
