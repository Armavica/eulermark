// Copyright (C) 2014 Jorge Aparicio

use std::iter::{AdditiveIterator,count};
use std::num::pow;

struct Digits(Vec<uint>);

impl Digits {
    fn to_uint(&self) -> uint {
        let &Digits(ref ds) = self;

        ds.iter().rev().fold(0, |acc, &d| 10 * acc + d)
    }

    fn power_sum(&self, powers: &[uint]) -> uint {
        let &Digits(ref ds) = self;

        ds.iter().fold(0, |acc, &d| acc + powers[d])
    }

    fn next(&mut self) {
        let &Digits(ref mut ds) = self;
        let size = ds.len();

        *ds.get_mut(0) += 1;

        for i in range(0, size) {
            if *ds.get(i) == 10 {
                *ds.get_mut(i) = 0;

                if i == size - 1 {
                    ds.push(1);
                } else {
                    *ds.get_mut(i + 1) += 1;
                }
            }
        }
    }

    fn bump(&mut self) {
        let &Digits(ref mut ds) = self;
        let size = ds.len();

        let mut done = false;
        for i in range(0, size) {
            if *ds.get(i) == 0 {
                continue;
            } else if !done {
                done = true;
                *ds.get_mut(i) = 0;

                if i == size - 1 {
                    ds.push(1);
                } else {
                    *ds.get_mut(i + 1) += 1;
                }
            } else if *ds.get(i) == 10 {
                *ds.get_mut(i) = 0;

                if i == size - 1 {
                    ds.push(1);
                } else {
                    *ds.get_mut(i + 1) += 1;
                }
            }
        }
    }
}

fn main() {
    let powers: Vec<uint> = range(0u, 10).map(|x| pow(x, 5)).collect();
    let mut digits = Digits(vec!(0, 1));

    let mut ans = Vec::new();

    let ceiling = pow(10u, count(1u, 1).skip_while(|n| {
        pow(10u, n - 1) < n * *powers.last().unwrap()
    }).next().unwrap());

    loop {
        let n = digits.to_uint();
        if n > ceiling {
            break;
        }

        let ps = digits.power_sum(powers.as_slice());

        if n == ps {
            ans.push(n);
        }

        if ps > n {
            digits.bump()
        } else {
            digits.next();
        }
    }

    println!("{}", ans.move_iter().sum());
}
