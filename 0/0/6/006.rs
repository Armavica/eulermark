// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;

fn main() {
  let sumSquared = square(range(1, 101).sum());
  let sumOfSquares = range(1, 101).map(square).sum();

  println!("{}", sumSquared - sumOfSquares);
}

fn square(a: int) -> uint {
  (a * a) as uint
}

