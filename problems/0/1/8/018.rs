// Copyright (C) 2014 Jorge Aparicio

use std::cmp::max;
use std::io::BufferedReader;
use std::io::fs::File;

fn main() {
    let mut content = BufferedReader::new(File::open(&Path::new("018.in")));

    let costs =
        content.
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

