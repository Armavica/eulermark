// Copyright (C) 2014 Jorge Aparicio

use std::iter::{AdditiveIterator,range_inclusive};
use std::num::pow;
use std::vec::from_elem;

fn main() {
    let n = 10000;
    let mut primes = ~[2];
    let mut sod = from_elem(n + 1, 0u);

    for i in range_inclusive(2u, n) {
        let factors = factorize(i, &mut primes);
        sod[i] = factors.divisors().iter().fold(0, |a, &b| a + b) - i;
    }

    println!("{}",
        range_inclusive(2u, n).filter(|&x| is_amicable(x, sod)).sum());
}

fn is_amicable(n: uint, sod: &[uint]) -> bool {
    let m = sod[n];

    m != n && m < sod.len() && sod[m] == n
}

// Factors(~[(a, x), (b, y)]) <-> a^x * b^y
struct Factors(~[(uint, uint)]);

fn factorize(mut n: uint, primes: &mut ~[uint]) -> Factors {
    let mut factors = ~[];

    for &prime in primes.iter() {
        if n == 1 {
            break;
        }

        if prime * prime > n {
            factors.push((n, 1));
            break;
        }

        let mut i = 0;
        while n % prime == 0 {
            i += 1;
            n /= prime;
        }

        if i != 0 {
            factors.push((prime, i));
        }
    }

    if n != 1 && n > *primes.last().unwrap() {
        primes.push(n);
    }

    Factors(factors)
}

impl Factors {
    fn divisors(&self) -> ~[uint] {
        let &Factors(ref s) = self;

        s.iter().fold(~[1], |x, &(b, n)| combine(x, range_inclusive(0, n).map(|x| pow(b, x)).to_owned_vec()))
    }
}

fn combine(xs: &[uint], ys: &[uint]) -> ~[uint] {
    let mut z = ~[];

    for &x in xs.iter() {
        for &y in ys.iter() {
            z.push(x * y)
        }
    }

    z
}

