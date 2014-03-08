// Copyright (C) 2013 Jorge Aparicio

extern crate num;

use num::Integer;

fn main() {
    println!("{}", factors(600851475143).last().unwrap().val0());
}

struct Factorize { num: uint, factor: uint }

fn factors(n: uint) -> Factorize {
    Factorize { num: n, factor: 2 }
}

impl Iterator<(uint, uint)> for Factorize {
    fn next(&mut self) -> Option<(uint, uint)> {
        if self.num == 1 {
            None
        } else {
            while self.num % self.factor != 0 {
                if self.factor * self.factor > self.num {
                    self.factor = self.num;
                    break;
                }

                self.factor += if self.factor.is_even() { 1 } else { 2 };
            }

            let mut exponent = 0;
            while self.num % self.factor == 0 {
                exponent += 1;
                self.num /= self.factor;
            }

            Some((self.factor, exponent))
        }
    }
}

