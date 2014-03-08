// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::num::pow;

fn main() {
    let sumSquared = pow(range(1, 101).sum(), 2);
    let sumOfSquares = range(1, 101).map(|x| pow(x, 2)).sum();

    println!("{}", sumSquared - sumOfSquares);
}

