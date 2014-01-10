// Copyright (C) 2013 Jorge Aparicio

fn main() {
  println!("{}", factorize(600851475143).last().unwrap());
}

struct Factorize { num: int, factor: int }

fn factorize(n: int) -> Factorize {
  Factorize { num: n, factor: 2 }
}

impl Iterator<int> for Factorize {
  fn next(&mut self) -> Option<int> {
    if self.num == 1 {
      None
    } else {
      while self.num % self.factor != 0 {
        self.factor += 1;

        if self.factor * self.factor > self.num {
          self.factor = self.num;
          break;
        }
      }

      self.num /= self.factor;
      Some(self.factor)
    }
  }
}

