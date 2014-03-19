// Copyright (C) 2014 Jorge Aparicio

use std::num::pow;
use std::vec_ng::Vec;

// Factors(vec!((a, x), (b, y))) <-> a^x * b^y
#[deriving(Eq,TotalEq,TotalOrd)]
struct Factors(Vec<(uint, int)>);

impl Factors {
    fn pow(&self, exp: int) -> Factors {
        let &Factors(ref s) = self;

        let mut out = Vec::new();

        for &(b, e) in s.iter() {
            out.push((b, e * exp));
        }

        Factors(out)
    }
}

impl Div<Factors,Factors> for Factors {
    fn div(&self, rhs: &Factors) -> Factors {
        let (&Factors(ref s), &Factors(ref r)) = (self, rhs);

        let (m, n) = (s.len(), r.len());
        let (mut i, mut j) = (0 ,0);

        let mut o = vec!();
        while i != m || j != n {

            if i == m {
                let &(b, y) = r.get(j);
                o.push((b, -y));
                j += 1;
            } else if j == n {
                o.push(*s.get(i));
                i += 1;
            } else {
                let (&(a, x), &(b, y)) = (s.get(i), r.get(j));

                if a > b {
                    o.push((b, -y));
                    j += 1;
                } else if a < b {
                    o.push((a, x));
                    i += 1;
                } else {
                    if x - y != 0 {
                        o.push((a, x - y));
                    }

                    i += 1;
                    j += 1;
                }
            }
        }

        Factors(o)
    }
}

impl Ord for Factors {
    fn lt(&self, rhs: &Factors) -> bool {
        let Factors(ref div) = self / *rhs;

        let (left, right) = div.iter().fold((1, 1), |(l, r), &(b, e)| {
            if e > 0 {
                (l * pow(b, e as uint), r)
            } else {
                (l, r * pow(b, -e as uint))
            }
        });

        left < right
    }
}

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

fn main() {
    let mut combinations = Vec::new();
    let mut primes = vec!(2);

    for a in range(2u, 101) {
        for b in range(2, 101) {
            combinations.push(factorize(a, &mut primes).pow(b));
        }
    }

    combinations.sort();
    combinations.dedup();

    println!("{}", combinations.len());
}

