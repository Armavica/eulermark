// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::iter::range_step;
use std::vec;

fn main() {
    println!("{}", primes(2000000).sum());
}

struct Primes { sieve: ~[bool], n: uint, i: uint }

fn primes(upper: uint) -> Primes {
    let n = (upper - 3) / 2;

    Primes { sieve: vec::from_elem(n, true), n: n, i: 0 }
}

impl Iterator<uint> for Primes {
    fn next(&mut self) -> Option<uint> {
        if self.i == 0 {
            self.i += 1;
            return Some(2);
        }

        while self.i <= self.n && !self.sieve[self.i - 1] {
            self.i += 1;
        }

        if self.i > self.n {
            return None;
        }

        let p = 2 * self.i + 1;
        self.i += 1;

        for j in range_step(p * p, 2 * self.n + 3, 2 * p) {
            self.sieve[(j - 3) / 2] = false;
        }

        Some(p)
    }
}

