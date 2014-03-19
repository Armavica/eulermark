// Copyright (C) 2014 Jorge Aparicio

use std::iter::count;
use std::mem::replace;
use std::vec_ng::Vec;

fn main() {
    let mut primes = vec!(2);
    let mut next = Factors(vec!((2, 1)));

    for n in count(3u, 1) {
        let curr = replace(&mut next, factorize(n, &mut primes));
        let triangle = next * curr * Factors(vec!((2, -1)));

        if triangle.number_of_divisors() > 500 {
            println!("{}", n * (n - 1) / 2);
            return;
        }
    }
}

// Factors(vec!((a, x), (b, y))) <-> a^x * b^y
struct Factors(Vec<(uint, int)>);

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
    fn number_of_divisors(&self) -> int {
        let &Factors(ref s) = self;

        s.iter().fold(1, |n, &(_, e)| n * (e + 1))
    }
}

impl Mul<Factors, Factors> for Factors {
    fn mul(&self, rhs: &Factors) -> Factors {
        let (&Factors(ref s), &Factors(ref r)) = (self, rhs);

        let (m, n) = (s.len(), r.len());
        let (mut i, mut j) = (0 ,0);

        let mut o = vec!();
        while i != m || j != n {
            if i == m {
                o.push(*r.get(j));
                j += 1;
            } else if j == n {
                o.push(*s.get(i));
                i += 1;
            } else {
                let (&(a, x), &(b, y)) = (s.get(i), r.get(j));

                if a > b {
                    o.push((b, y));
                    j += 1;
                } else if a < b {
                    o.push((a, x));
                    i += 1;
                } else {
                    if x + y > 0 {
                        o.push((a, x + y));
                    }

                    i += 1;
                    j += 1;
                }
            }
        }

        Factors(o)
    }
}

