// Copyright (C) 2014 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::vec_ng::Vec;

fn to_number(digits: &[u8]) -> uint {
    digits.iter().fold(0, |acc, &d| 10 * acc + d as uint)
}

fn pandigital_products(digits: &[u8]) -> Vec<uint> {
    let mut out = Vec::new();

    for m in range(1u, 5) {
        for n in range(m + 1, 9) {
            let f1 = to_number(digits.slice_to(m));
            let f2 = to_number(digits.slice(m, n));
            let p = to_number(digits.slice_from(n));

            if f1 * f2 == p {
                out.push(p);
            }
        }
    }

    out
}

fn main() {
    let digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut products = Vec::new();

    for perm in digits.permutations() {
        products.push_all_move(pandigital_products(perm.as_slice()));
    }

    products.sort();
    products.dedup();

    println!("{}", products.move_iter().sum());
}

