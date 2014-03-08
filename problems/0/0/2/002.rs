// Copyright (C) 2013 Jorge Aparicio

extern crate num;

use num::Integer;
use std::iter::AdditiveIterator;
use std::mem::replace;

fn main() {
    println!("{}", fibonacci(1, 2).take_while(|&x| x < 4_000_000).filter(|x| x.is_even()).sum());
}

struct Fibonacci { curr: uint, next: uint }

fn fibonacci(first: uint, second: uint) -> Fibonacci {
    Fibonacci { curr: first, next: second }
}

impl Iterator<uint> for Fibonacci {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

