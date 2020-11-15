use std::io::{self, BufRead};

fn factorial_digits(n: u64) -> u64 {
    let mut c: u64 = 1;
    let mut f: f32 = 1.0;
    for i in 2..=n {
        f *= i as f32;
        while f > 10.0 {
            f /= 10.0;
            c += 1;
        }
    }
    c
}

fn main() {
    let stdin = io::stdin();
    for num in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<u64>().unwrap())
    {
        println!("{}", factorial_digits(num));
    }
}
