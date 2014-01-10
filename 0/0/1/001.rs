// Copyright (C) 2013 Jorge Aparicio

use std::iter::AdditiveIterator;

fn main() {
  println!("{}", range(0, 1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum());
}

