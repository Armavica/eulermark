// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::Integer;
use std::iter::{count,range_inclusive};

fn main() {
    let limit = 999;
    let (mut ans, mut max) = (0, 0);

    for a in range_inclusive(-limit, limit) {
        for b in range_inclusive(-limit, limit) {
            let len = prime_length(a, b);

            if len > max {
                max = len;
                ans = a * b;
            }
        }
    }

    println!("{}", ans);
}

fn prime_length(a:int, b: int) -> uint {
    count(0u, 1).skip_while(|&n| {
        let p = polyval(a, b, n as int);

        p > 1 && is_prime(p as uint)
    }).next().unwrap()
}

fn polyval(a: int, b: int, n: int) -> int {
    n * n + a * n + b
}

fn is_prime(n: uint) -> bool {
    let mut p = 2;

    loop {
        if p * p > n { break }

        if n % p == 0 { return false }

        p += if p.is_even() { 1 } else { 2 };
    }

    true
}

