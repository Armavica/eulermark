// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_inclusive;
use std::num::pow;
use std::vec::from_elem;

fn main() {
    let limit = 28123;
    let mut primes = ~[2];
    let mut sopd = from_elem(limit + 1, 0u);

    for i in range_inclusive(2u, limit) {
        sopd[i] = factorize(i, &mut primes).divisors().iter().fold(0, |a, &b| a + b) - i;
    }

    let abundant_numbers = range_inclusive(2u, limit).filter(|&x| is_abundant(x, sopd)).to_owned_vec();

    let mut abundant_sums = from_elem(limit + 1, false);

    for i in range(0, abundant_numbers.len()) {
        for j in range_inclusive(0, i) {
            let s = abundant_numbers[i] + abundant_numbers[j];

            if s < limit {
                abundant_sums[s] = true;
            }
        }
    }

    println!("{}", range(0, limit).fold(0, |a, n| {
        if abundant_sums[n] { a } else { a + n }
    }))
}

fn is_abundant(n: uint, sopd: &[uint]) -> bool {
    sopd[n] > n
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

