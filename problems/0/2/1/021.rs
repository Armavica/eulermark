// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::num::pow;

fn main() {
    let n = 10000;
    let mut primes = vec!(2);
    let mut sod = Vec::from_elem(n + 1, 0u);

    for i in range(2u, n + 1) {
        let factors = factorize(i, &mut primes);
        *sod.get_mut(i) = factors.divisors().iter().fold(0, |a, &b| a + b) - i;
    }

    println!("{}", range(2u, n + 1).filter(|&x| is_amicable(x, sod.as_slice())).sum());
}

fn is_amicable(n: uint, sod: &[uint]) -> bool {
    let m = sod[n];

    m != n && m < sod.len() && sod[m] == n
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

