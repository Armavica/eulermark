// Copyright (C) 2014 Jorge Aparicio

extern crate num;

use num::Integer;

fn main() {
    let upper = 1000000;
    let mut memo = Vec::from_elem(upper, 0u);

    println!("{}", range(1, upper).max_by(|&x| collatz_length(x, memo.as_mut_slice())).unwrap());
}

fn collatz_length(n: uint, memo: &mut [uint]) -> uint {
    match memo.get(n) {
        Some(&length) if length != 0 => length,
        _ => {
            let length = 1 + if n == 1 {
                0
            } else if n.is_even() {
                collatz_length(n / 2, memo)
            } else {
                collatz_length(3 * n + 1, memo)
            };

            if n < memo.len() {
                memo[n] = length;
            }

            length
        }
    }
}

