// Copyright (C) 2014 Jorge Aparicio

use std::num::pow;

static P: uint = 1000;

fn main() {
    for c in range(P / 3 + 1, P / 2) {
        for b in range((P - c) / 2 + 1, c) {
            let a = P - b - c;

            if pow(a, 2) + pow(b, 2) == pow(c, 2) {
                println!("{}", a * b * c);

                return;
            }
        }
    }
}

