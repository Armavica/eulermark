// Copyright (C) 2014 Jorge Aparicio

use std::char::from_digit;
use std::iter::MultiplicativeIterator;

fn main() {
    println!("{}", nth_perm(999999, &mut range(0u, 10).collect()));
}

fn nth_perm(mut m: uint, digits: &mut Vec<uint>) -> ~str {
    let mut out = StrBuf::new();

    while digits.len() > 0 {
        let n = digits.len();
        let p = factorial(n - 1);
        let i = m / p;
        m = m % p;

        out.push_char(from_digit(digits.remove(i).unwrap(), 10).unwrap());
    }

    out.into_owned()
}

fn factorial(n: uint) -> uint {
    range(1, n + 1).product()
}

