// Copyright (C) 2014 Jorge Aparicio

static size: uint = 1000000;

fn main() {
  let mut collatz_lenghts = ~[0, ..size];
  let (mut max_length, mut ans) = (0, 0);

  for i in range(1, size) {
    let length = collatz(i, collatz_lenghts);

    if length > max_length {
      max_length = length;
      ans = i;
    }
  }

  println!("{}", ans);
}

fn collatz(n: uint, memo: &mut [uint]) -> uint {
  if n < size && memo[n] != 0 {
    memo[n]
  } else {
    let out = 1 + if n == 1 {
      0
    } else if n.is_even() {
      collatz(n / 2, memo)
    } else {
      collatz(3 * n + 1, memo)
    };

    if n < size {
      memo[n] = out;
    }

    out
  }
}

