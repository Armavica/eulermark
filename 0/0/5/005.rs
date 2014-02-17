// Copyright (C) 2014 Jorge Aparicio

use std::num::lcm;

fn main() {
    println!("{}", range(2, 21u).fold(1, |a, b| lcm(a, b)));
}

