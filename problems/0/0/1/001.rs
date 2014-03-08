// Copyright (C) 2013 Jorge Aparicio

use std::iter::AdditiveIterator;

fn main() {
    println!("{}", range(0, 1_000u).filter(|x| x % 3 == 0 || x % 5 == 0).sum());
}

