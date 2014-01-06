// Copyright (C) 2014 Jorge Aparicio

fn main() {
  println(range(2, 21).
          fold(1, |a, b| lcm(a, b)).
          to_str());
}

fn lcm(a: int, b: int) -> int {
  a * b / gcd(a, b)
}

fn gcd(mut a: int, mut b: int) -> int {
  if a < b {
    let t = a;
    a = b;
    b = t;
  }

  loop {
    let r = a % b;

    if r == 0 {
      break;
    }

    a = b;
    b = r;
  }

  b
}

