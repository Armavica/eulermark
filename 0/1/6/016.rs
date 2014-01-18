// Copyright (C) 2014 Jorge Aparicio

extern mod extra;
use extra::bigint::BigInt;
use extra::bigint::ToBigInt;
use std::char::to_digit;
use std::iter::AdditiveIterator;

fn main() {
  let b = (2 as int).to_bigint().unwrap();
  let n = pow(b, 1000).to_str();
  println!("{}", n.chars().filter_map(|c| to_digit(c, 10)).sum());
}

fn pow(b: BigInt, n: uint) -> BigInt {
  match n {
    0 => (1 as int).to_bigint().unwrap(),
    1 => b,
    _ => pow(b.clone(), n % 2) * pow(b * b, n / 2)
  }
}

