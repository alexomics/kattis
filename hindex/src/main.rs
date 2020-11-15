use std::io::{self, BufRead};

fn top(v: &Vec<u32>) -> Option<u32> {
    v.last().copied()
}

fn main() {
    let mut cites: Vec<u32> = Vec::new();
    let stdin = io::stdin();
    let _n: u32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    for num in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<u32>().unwrap())
    {
        cites.push(num);
    }
    cites.sort();
    let mut h = 0;

    while cites.len() > 0 && top(&cites).unwrap() > h {
        h += 1;
        cites.pop();
    }
    println!("{:?}", h);
}
