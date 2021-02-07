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

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize, W: usize,
        S: [Chars; H],
    };
    let mut ans = 4;

    for y in 0..H {
        for x in 0..W {
            if S[y][x] == '#' {
                // right
                if S[y + 1][x + 1] == '#' && (S[y][x + 1] == '.' || S[y + 1][x] == '.') {
                    ans += 2;
                }

                // left
                if S[y + 1][x - 1] == '#' && (S[y][x - 1] == '.' || S[y + 1][x] == '.') {
                    ans += 2;
                }
            }
        }
    }

    echo!(ans);

    // let mut start_x = 0;
    // let mut start_y = 0;
    // 'outer: for y in 0..H {
    //     for x in 0..W {
    //         if S[y][x] == '#' {
    //             start_x = x;
    //             start_y = y;
    //             break 'outer;
    //         }
    //     }
    // }

    // let (mut x, mut y) = (start_x, start_y);
    // // right
    // loop {
    //     if S[y - 1][x + 1] == '#' {
    //         ans += 2;
    //         x = x + 1;
    //         y = y - 1;
    //         continue;
    //     }
    //     if S[y][x + 1] == '#' {
    //         x = x + 1;
    //         continue;
    //     }
    //     if S[y + 1][x + 1] == '#' {
    //         x = x + 1;
    //         y = y + 1;
    //         ans += 2;
    //         continue;
    //     }
    //     break;
    // }

    // // down
    // loop {
    //     if S[y + 1][x + 1] == '#' {
    //         ans += 2;
    //         x = x + 1;
    //         y = y + 1;
    //         continue;
    //     }
    //     if S[y + 1][x] == '#' {
    //         y = y + 1;
    //         continue;
    //     }
    //     if S[y + 1][x - 1] == '#' {
    //         x = x - 1;
    //         y = y + 1;
    //         ans += 2;
    //         continue;
    //     }
    //     break;
    // }

    // // left
    // loop {
    //     if S[y + 1][x - 1] == '#' {
    //         ans += 2;
    //         x = x - 1;
    //         y = y + 1;
    //         continue;
    //     }
    //     if S[y][x - 1] == '#' {
    //         x = x - 1;
    //         continue;
    //     }
    //     if S[y - 1][x - 1] == '#' {
    //         x = x - 1;
    //         y = y - 1;
    //         ans += 2;
    //         continue;
    //     }
    //     break;
    // }

    // // up
    // loop {
    //     if S[y - 1][x - 1] == '#' {
    //         ans += 2;
    //         x = x - 1;
    //         y = y - 1;
    //         continue;
    //     }
    //     if S[y - 1][x] == '#' {
    //         y = y - 1;
    //         if (x, y) == (start_x, start_y) {
    //             break;
    //         }
    //         continue;
    //     }
    //     if S[y - 1][x + 1] == '#' {
    //         x = x + 1;
    //         y = y - 1;
    //         ans += 2;
    //         continue;
    //     }
    //     break;
    // }
}
