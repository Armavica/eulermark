// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;
use std::num::max;

fn main() {
    let contents = File::open(&Path::new("018.in")).read_to_str();

    let costs =
        contents.
        lines().
        map(|l| l.words().filter_map(|w| from_str::<uint>(w)).to_owned_vec()).
        fold(~[0], |a, b| {
            let n = b.len();
            let mut c = ~[];

            for i in range(0, n) {
                if i == 0 {
                    c.push(b[i] + a[i]);
                } else if i == n - 1 {
                    c.push(b[i] + a[i - 1]);
                } else {
                    c.push(b[i] + max(a[i - 1], a[i]));
                }
            }

            c
        });

    println!("{}", *costs.iter().max().unwrap());
}

