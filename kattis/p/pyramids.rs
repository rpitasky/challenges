use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut blocks = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let mut i = 1;
    loop {
        if i * i > blocks {
            println!("{}", (i - 1) / 2);
            break;
        } else {
            blocks -= dbg!(i * i);
        }

        i += 2;
    }
}
