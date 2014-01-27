// Copyright (C) 2013 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::util::replace;

fn main() {
    println!("{}",
        fibonacci().take_while(|&x| x < 4000000).filter(|x| x % 2 == 0).sum());
}

struct Fibonacci { curr: uint, next: uint }

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 2 }
}

impl Iterator<uint> for Fibonacci {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);
        Some(replace(&mut self.curr, new_curr))
    }
}

