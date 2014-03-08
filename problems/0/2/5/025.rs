// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use std::mem::replace;
use std::num::One;
use std::num::Zero;

fn main() {
    println!("{}", fibonacci().enumerate().skip_while(|&(_, ref n)| number_of_digits(n.clone()) < 1000).next().unwrap().val0() + 1);
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

