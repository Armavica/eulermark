// Copyright (C) 2013 Jorge Aparicio

fn main() {
  let mut max = 0;

  for a in range(100, 1000) {
    for b in range(100, a) {
      let p = a * b;

      if p > max && isPalindrome(p) {
        max = p;
      }
    }
  }

  println!("{}", max);
}

fn isPalindrome(n: int) -> bool {
  let (mut reversed, mut tmp) = (0, n);

  while tmp != 0 {
    reversed = 10*reversed + tmp%10;
    tmp /= 10;
  }

  reversed == n
}

