use std::io::{self, BufRead};

fn do_add(s: String) -> i32 {
    s.split("+").map(|n| n.parse::<i32>().unwrap()).sum()
}

fn main() {

    let stdin = io::stdin();
    let skip = String::from("P=NP");
    for (i, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        if i == 0 {
            continue;
        }
        match line == skip {
            true => {println!("skipped")},
            _ => {println!("{}", do_add(line))}
        }
    }
}
