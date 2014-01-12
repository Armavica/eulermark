// Copyright (C) 2014 Jorge Aparicio

static P: int = 1000;

fn main() {
  for c in range(P / 3 + 1, P / 2) {
    for b in range((P - c) / 2 + 1, c) {
      let a = P - b - c;

      if a*a + b*b == c*c {
        println!("{}", a * b * c);

        return;
      }
    }
  }
}

