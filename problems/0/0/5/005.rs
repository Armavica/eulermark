// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::lcm;

fn main() {
    println!("{}", range(2, 21u).fold(1, |a, b| lcm(a, b)));
}

