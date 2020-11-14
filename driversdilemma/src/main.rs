use std::io::{self, BufRead};

#[derive(Debug)]
struct Car {
    capacity: f32,
    rate_of_loss: f32,
    remaining: f32,
    efficiancy: Vec<MPG>,
}

#[derive(Debug)]
struct MPG {
    mph: f32,
    mpg: f32,
}

impl Car {
    fn in_range(&self, distance: f32) -> f32 {
        let mut res: f32 = -1.0;
        let volume = self.capacity * self.remaining;
        for entry in &self.efficiancy {
            let time = distance / entry.mph;
            let total_loss = time * self.rate_of_loss;
            let max_distance = (volume - total_loss) * entry.mpg;
            if max_distance > distance {
                res = res.max(entry.mph);
            }
        }
        res
    }
}

fn main() {
    let remaining: f32 = 0.5;
    let mut c: f32 = 0.0;
    let mut x: f32 = 0.0;
    let mut m: f32 = 0.0;
    let mut efficiancies: Vec<MPG> = Vec::new();

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        let nums: Vec<f32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if i == 0 {
            c = nums[0];
            x = nums[1];
            m = nums[2];
        } else {
            efficiancies.push(MPG {
                mph: nums[0],
                mpg: nums[1],
            });
        }
    }
    let car = Car {
        capacity: c,
        rate_of_loss: x,
        remaining: remaining,
        efficiancy: efficiancies,
    };
    let y = car.in_range(m);
    if y > 0.0 {
        println!("YES {:.0}", y);
    } else {
        println!("NO");
    }
}
