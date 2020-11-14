use std::io::{self, BufRead};

fn main() {
    let remaining: f32 = 0.5;
    let mut x: f32 = 0.0;
    let mut m: f32 = 0.0;
    let mut res: f32 = -1.0;
    let mut volume: f32 = 0.0;

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let nums: Vec<f32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if i == 0 {
            x = nums[1];
            m = nums[2];
            volume = nums[0] * remaining;
        } else {
            let time = m / nums[0];
            let total_loss = time * x;
            let max_distance = (volume - total_loss) * nums[1];
            if max_distance > m {
                res = res.max(nums[0]);
            }
        }
    }
    if res > 0.0 {
        println!("YES {:.0}", res);
    } else {
        println!("NO");
    }
}
