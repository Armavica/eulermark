// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::bigint::BigInt;
use std::io::BufferedReader;
use std::io::fs::File;
use std::iter::AdditiveIterator;

fn main() {
    let mut content = BufferedReader::new(File::open(&Path::new("013.in")));

    let sum = content.
        lines().
        filter_map(|line| from_str::<BigInt>(line.trim_right())).
        sum().
        to_str();

    println!("{}", sum.slice(0, 10));
}

