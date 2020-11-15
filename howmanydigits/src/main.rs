use std::io::{self, BufRead};

fn main() {
    // https://math.stackexchange.com/questions/1075422/how-many-digits-are-there-in-100
    let mut arr: [f64; 1000001] = [0.0; 1000001];
    arr[0] = 0.0;
    for i in 1..=1000000 {
        arr[i] = arr[i - 1] + (i as f64).log10();
    }
    let stdin = io::stdin();
    for num in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<usize>().unwrap())
    {
        println!("{}", (arr[num] + 1.0) as u32);
    }
}
