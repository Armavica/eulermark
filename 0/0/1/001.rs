// Copyright (C) 2013 Jorge Aparicio

use std::iter::AdditiveIterator;

fn main() {
  let limit = 1000;

  println(range(0, limit).
          filter(|x| x % 3 == 0 || x % 5 == 0).
          sum().
          to_str());
}

