// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_step_inclusive;

fn main() {
    let limit = 1000;
    let mut primes = vec!();
    let mut sieve = Vec::from_elem(limit + 1, true);

    for i in range(2u, 1000) {
        if *sieve.get(i) {
            primes.push(i);

            for j in range_step_inclusive(i * i, limit, i) {
                *sieve.get_mut(j) = false;
            }
        }
    }

    println!("{}", primes.move_iter().max_by(|&a| cycle_size(a)).unwrap());
}

fn cycle_size(den: uint) -> uint {
    let mut num = 1;
    let mut size = 0;

    loop {
        while num < den {
            num *= 10;
        }

        num %= den;
        size += 1;

        if num < 2 {
            if num == 0 {
                size = 0;
            }

            break;
        }
    }


    size
}

