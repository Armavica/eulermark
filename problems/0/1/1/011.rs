// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;
use std::vec_ng::Vec;

static STRIDE: int = 4;

fn main() {
    let content = File::open(&Path::new("011.in")).read_to_str().unwrap();

    let grid: Vec<Vec<uint>> =
        content.
        lines().
        map(|line| line.words().filter_map(from_str).collect()).
        collect();

    let (nr, nc) = (grid.len() as int, grid.get(0).len() as int);
    let directions = [(0, 1), (1, 0), (1, 1), (-1, 1)];

    let mut max = 0;
    for &(y_step, x_step) in directions.iter() {
        for r in range(0, nr) {
            let y_max = r + STRIDE * y_step;
            if y_max > nr || y_max < -1 { continue }

            for c in range(0, nc) {
                let x_max = c + STRIDE * x_step;
                if x_max > nc || x_max < -1 { continue }

                let mut tmp = *grid.get(r as uint).get(c as uint);
                for i in range(1, STRIDE) {
                    let x = (c + i * x_step) as uint;
                    let y = (r + i * y_step) as uint;

                    tmp *= *grid.get(y).get(x);
                }

                if tmp > max { max = tmp }
            }
        }
    }

    println!("{}", max);
}

