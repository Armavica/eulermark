// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::bigint::BigUint;
use std::mem::replace;
use std::num::{One, Zero};

fn main() {
    let thousand_digits = BigUint::parse_bytes(Vec::from_fn(1000, |i| if i==0 {1} else {0} + 48).as_slice(), 10).unwrap();
    println!("{}",
              fibonacci()
             .enumerate()
             .skip_while(|&(_, ref n)| n < &thousand_digits)
             .next().unwrap().val0());
}

struct Fibonacci { curr: BigUint, next: BigUint }

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: Zero::zero(), next: One::one() }
}

impl Iterator<BigUint> for Fibonacci {
    fn next(&mut self) -> Option<BigUint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);
        Some(replace(&mut self.curr, new_curr))
    }
}

