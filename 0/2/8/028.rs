// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_step_inclusive;

fn main() {
    println!("{}",
        range_step_inclusive(3, 1001, 2).fold((1, 1), |(acc, corner), size| {
            let step = size - 1;

            range(0, 4).fold((acc, corner), |(a, c), _| {
                let next_c = c + step;
                (a + next_c, next_c)
            })
        }).val0());
}
