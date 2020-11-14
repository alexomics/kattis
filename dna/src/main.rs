use std::io;
use std::io::prelude::*;
use std::str;

fn main() {
    let mut _n: Vec<u8> = Vec::new();
    let mut s: Vec<u8> = Vec::new();
    let mut done = false;
    let mut n: i32 = 0;

    for b in io::stdin().bytes().map(|x| x.unwrap()) {
        if !done {
            if b == 10 {
                done = true;
                n = str::from_utf8(&_n).unwrap().parse::<i32>().unwrap();
                continue;
            }
            _n.push(b);
        } else {
            if b == 10 {
                continue;
            }
            s.push(b);
        }
    }

    let mut i;
    let mut mutations = 0;
    let mut prefix_flipped = false;

    for idx in (0..n).rev() {
        i = idx as usize;

        if prefix_flipped && s[i] == 66 {
            continue;
        }
        if !prefix_flipped && s[i] == 65 {
            continue;
        }
        if i == 0 {
            mutations += 1;
            continue;
        }

        mutations += 1;

        if prefix_flipped {
            if s[i - 1] == 65 {
                prefix_flipped = false;
            }
        } else {
            if s[i - 1] == 66 {
                prefix_flipped = true;
            }
        }
    }
    println!("{}", mutations);
}
