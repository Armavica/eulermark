// Copyright (C) 2014 Jorge Aparicio

use std::cmp::max;
use std::io::fs::File;
use std::vec_ng::Vec;

fn main() {
    let content = File::open(&Path::new("018.in")).read_to_str().unwrap();

    let costs =
        content.
        lines().
        map(|l| l.words().filter_map(from_str).collect::<Vec<uint>>()).
        fold(vec!(0), |a, b| {
            let n = b.len();
            let mut c = Vec::with_capacity(n);

            for i in range(0, n) {
                if i == 0 {
                    c.push(b.get(i) + *a.get(i));
                } else if i == n - 1 {
                    c.push(b.get(i) + *a.get(i - 1));
                } else {
                    c.push(b.get(i) + *max(a.get(i - 1), a.get(i)));
                }
            }

            c
        });

    println!("{}", *costs.iter().max().unwrap());
}

