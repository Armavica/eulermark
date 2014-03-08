// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use std::iter::AdditiveIterator;
use std::iter::MultiplicativeIterator;
use std::iter::range_inclusive;
use std::num::One;

fn main() {
    println!("{}",
        factorial(100).
        to_str().
        chars().
        filter_map(|c| c.to_digit(10)).
        sum());
}

fn factorial(n: uint) -> BigUint {
    range_inclusive(One::one(), n.to_biguint().unwrap()).product()
}

