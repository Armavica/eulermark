// Copyright (C) 2014 Jorge Aparicio

use std::char::to_digit;
use std::io::fs::File;

static stride: uint = 5;

fn main() {
    let (mut max, mut pos) = (0, 0);
    let mut factors: [uint, ..stride] = [0, ..stride];

    let contents = File::open(&Path::new("008.in")).read_to_str();

    for line in contents.lines() {
        for n in line.chars().map(|c| to_digit(c, 10).unwrap()) {
            factors[pos] = n;

            let prod = factors.iter().fold(1, |a, &b| a * b);

            if prod > max {
                max = prod;
            }

            pos = (pos + 1) % stride;
        }
    }

    println!("{}", max);
}

