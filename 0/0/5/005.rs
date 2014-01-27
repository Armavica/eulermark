// Copyright (C) 2014 Jorge Aparicio

use std::util::{replace,swap};

fn main() {
    println!("{}", range(2, 21u).fold(1, |a, b| lcm(a, b)));
}

fn lcm(a: uint, b: uint) -> uint {
    a * b / gcd(a, b)
}

fn gcd(mut a: uint, mut b: uint) -> uint {
    if a < b {
        swap(&mut a, &mut b);
    }

    loop {
        let r = a % b;

        if r == 0 {
            break;
        }

        a = replace(&mut b, r);
    }

    b
}

