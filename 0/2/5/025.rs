// Copyright (C) 2014 Jorge Aparicio

extern mod extra;

use extra::bigint::BigUint;
use extra::bigint::ToBigUint;
use std::num::One;
use std::num::Zero;
use std::util::replace;

fn main() {
    println!("{}", fibonacci().enumerate().skip_while(|&(_, ref n)| number_of_digits(n.clone()) < 1000).next().unwrap().n0() + 1);
}

fn number_of_digits(mut n: BigUint) -> uint {
    let mut nod = 0;

    while !n.is_zero() {
        nod += 1;
        n = n / 10u.to_biguint().unwrap();
    }

    nod
}

struct Fibonacci { curr: BigUint, next: BigUint }

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: One::one(), next: One::one() }
}

impl Iterator<BigUint> for Fibonacci {
    fn next(&mut self) -> Option<BigUint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);
        Some(replace(&mut self.curr, new_curr))
    }
}

