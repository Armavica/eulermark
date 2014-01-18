// Copyright (C) 2014 Jorge Aparicio

extern mod extra;
use extra::bigint::ToBigInt;
use std::iter::AdditiveIterator;
use std::iter::MultiplicativeIterator;

fn main() {
  let l = (1 as int).to_bigint().unwrap();
  let h = (101 as int).to_bigint().unwrap();
  let factorial = range(l, h).product().to_str();
  let sum = factorial.chars().map(|c| c.to_digit(10).unwrap()).sum();

  println!("{}", sum);
}

