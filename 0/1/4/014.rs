// Copyright (C) 2014 Jorge Aparicio

use std::vec;

fn main() {
    let upper = 1000000;
    let mut memo = vec::from_elem(upper, 0u);

    println!("{}",
        range(1, upper).max_by(|&x| collatz_length(x, &mut memo)).unwrap());
}

fn collatz_length(n: uint, memo: &mut ~[uint]) -> uint {
    let size = memo.len();

    if n < size && memo[n] != 0 {
        memo[n]
    } else {
        let out = 1 + if n == 1 {
            0
        } else if n.is_even() {
            collatz_length(n / 2, memo)
        } else {
            collatz_length(3 * n + 1, memo)
        };

        if n < size {
            memo[n] = out;
        }

        out
    }
}

