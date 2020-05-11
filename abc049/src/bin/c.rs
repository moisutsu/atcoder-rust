use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s: String
    }
    loop {
        match s.as_str() {
            w if w.ends_with("dream") => s.truncate(s.len() - "dream".len()),
            w if w.ends_with("dreamer") => s.truncate(s.len() - "dreamer".len()),
            w if w.ends_with("erase") => s.truncate(s.len() - "erase".len()),
            w if w.ends_with("eraser") => s.truncate(s.len() - "eraser".len()),
            _ => break,
        }
    }
    println!("{}",
        if s == "" { "YES" } else { "NO" }
    )
}
