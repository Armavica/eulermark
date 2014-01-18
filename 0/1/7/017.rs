// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;

fn main() {
  println!("{}", range(1u, 1001).map(|n| wordify(n)).sum());
}

fn wordify(n: uint) -> uint {
  if n < 10 {
    match n {
      1 => "one".len(),
      2 => "two".len(),
      3 => "three".len(),
      4 => "four".len(),
      5 => "five".len(),
      6 => "six".len(),
      7 => "seven".len(),
      8 => "eight".len(),
      _ => "nine".len()
    }
  } else if n < 20 {
    match n {
      10 => "ten".len(),
      11 => "eleven".len(),
      12 => "twelve".len(),
      13 => "thirteen".len(),
      15 => "fifteen".len(),
      18 => "eighteen".len(),
      _ => wordify(n % 10) + "teen".len()
    }
  } else if n < 100 {
    let u = n % 10;
    if u == 0 {
      match n {
        20 => "twenty".len(),
        30 => "thirty".len(),
        40 => "forty".len(),
        50 => "fifty".len(),
        80 => "eighty".len(),
        _ => wordify(n / 10) + "ty".len()
      }
    } else {
      let t = n - u;
      wordify(t) + wordify(u)
    }
  } else if n < 1000 {
    let tau = n % 100;
    if tau == 0 {
      wordify(n / 100) + "hundred".len()
    } else {
      let h = n - tau;
      wordify(h) + "and".len() + wordify(tau)
    }
  } else {
    wordify(n / 1000) + "thousand".len()
  }
}

