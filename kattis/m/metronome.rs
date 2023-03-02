use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!(
        "{}",
        (lines.next().unwrap().unwrap().parse::<f32>().unwrap()) / 4.0
    )
}
