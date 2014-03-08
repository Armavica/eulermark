// Copyright (C) 2014 Jorge Aparicio

use std::char::to_digit;
use std::io::BufferedReader;
use std::io::fs::File;

static STRIDE: uint = 5;

fn main() {
    let (mut max, mut pos) = (0, 0);
    let mut factors: [uint, ..STRIDE] = [0, ..STRIDE];

    let mut content = match File::open(&Path::new("008.in")) {
        Err(_)   => fail!("couldn't find input file"),
        Ok(file) => BufferedReader::new(file),
    };

    for char in content.chars() {
        if char != '\n' {
            factors[pos] = to_digit(char, 10).unwrap();

            let prod = factors.iter().fold(1, |a, &b| a * b);

            if prod > max {
                max = prod;
            }

            pos = (pos + 1) % STRIDE;
        }

    }

    println!("{}", max);
}

