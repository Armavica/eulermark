// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_step;

static size: int = 100000;
static target: int = 10000;

fn main() {
  let mut sieve: [bool, ..size] = [true, ..size];
  let mut count = 1;

  for i in range(0, size) {
    if sieve[i] {
      let p = 2*i + 3;

      if count == target {
        println!("{}", p);
        break;
      }

      for j in range_step(p * p, 2*size + 3, 2*p) {
        sieve[(j - 3) / 2] = false;
      }

      count += 1;
    }
  }
}

