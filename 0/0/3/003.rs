// Copyright (C) 2013 Jorge Aparicio

fn main() {
    println!("{}", factorize(600851475143).last().unwrap());
}

struct Factorize { num: uint, factor: uint }

fn factorize(n: uint) -> Factorize {
    Factorize { num: n, factor: 2 }
}

impl Iterator<uint> for Factorize {
    fn next(&mut self) -> Option<uint> {
        if self.num == 1 {
            None
        } else {
            while self.num % self.factor != 0 {
                if self.factor * self.factor > self.num {
                    self.factor = self.num;
                    break;
                }

                self.factor += if self.factor % 2 == 0 { 1 } else { 2 };
            }

            self.num /= self.factor;

            Some(self.factor)
        }
    }
}

