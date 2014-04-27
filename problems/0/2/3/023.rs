// Copyright (C) 2014 Jorge Aparicio

use std::num::pow;

fn main() {
    let limit = 28123;
    let mut primes = vec!(2);
    let mut sopd = Vec::from_elem(limit + 1, 0u);

    for i in range(2, limit + 1) {
        *sopd.get_mut(i) = factorize(i, &mut primes).divisors().iter().fold(0, |a, &b| a + b) - i;
    }

    let abundant_numbers: Vec<uint> = range(2, limit + 1).filter(|&x| {
        is_abundant(x, sopd.as_slice())
    }).collect();

    let mut abundant_sums = Vec::from_elem(limit + 1, false);

    for i in range(0, abundant_numbers.len()) {
        for j in range(0, i + 1) {
            let s = abundant_numbers.get(i) + *abundant_numbers.get(j);

            if s < limit {
                *abundant_sums.get_mut(s) = true;
            }
        }
    }

    println!("{}", range(0, limit).fold(0, |a, n| {
        if *abundant_sums.get(n) { a } else { a + n }
    }))
}

fn is_abundant(n: uint, sopd: &[uint]) -> bool {
    sopd[n] > n
}

// Factors(vec!((a, x), (b, y))) <-> a^x * b^y
struct Factors(Vec<(uint, uint)>);

fn factorize(mut n: uint, primes: &mut Vec<uint>) -> Factors {
    let mut factors = vec!();

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
    fn divisors(&self) -> Vec<uint> {
        let &Factors(ref s) = self;

        s.iter().fold(vec!(1), |xs, &(b, n)| {
            let ys: Vec<uint> = range(0, n + 1).map(|x| pow(b, x)).collect();

            combine(xs.as_slice(), ys.as_slice())
        })
    }
}

fn combine(xs: &[uint], ys: &[uint]) -> Vec<uint> {
    let mut z = vec!();

    for &x in xs.iter() {
        for &y in ys.iter() {
            z.push(x * y)
        }
    }

    z
}

