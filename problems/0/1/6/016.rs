// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use std::char::to_digit;
use std::iter::AdditiveIterator;
use std::num::One;

fn main() {
    println!("{}",
        pow(2u.to_biguint().unwrap(), 1000).
        to_str().
        chars().
        filter_map(|c| to_digit(c, 10)).
        sum());
}

fn pow(b: BigUint, n: uint) -> BigUint {
    match n {
        0 => One::one(),
        1 => b,
        n if n%2 == 0 => pow(b*b, n/2),
        n => pow(b*b, n/2) * b
    }
}

