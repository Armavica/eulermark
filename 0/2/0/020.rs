// Copyright (C) 2014 Jorge Aparicio

extern mod extra;

use extra::bigint::BigUint;
use extra::bigint::ToBigUint;
use std::iter::AdditiveIterator;
use std::iter::MultiplicativeIterator;
use std::iter::range_inclusive;
use std::num::One;

fn main() {
    println!("{}",
        factorial(100).
        to_str().
        chars().
        map(|c| c.to_digit(10).unwrap()).
        sum());
}

fn factorial(n: uint) -> BigUint {
    range_inclusive(One::one(), n.to_biguint().unwrap()).product()
}

