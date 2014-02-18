// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;
use std::io::BufferedReader;

static stride: uint = 4;

fn main() {
    let mut content = BufferedReader::new(File::open(&Path::new("011.in")));

    let grid =
        content.
        lines().
        map(|line| line.words().filter_map(from_str::<uint>).to_owned_vec()).
        to_owned_vec();

    let (nr, nc) = (grid.len(), grid[0].len());
    let directions = ~[(0, 1), (1, 0), (1, 1), (-1, 1)];

    let mut max= 0;
    for &(y, x) in directions.iter() {
        for r in range(0, nr) {
            if r + stride * y > nr { continue }

            for c in range(0, nc) {
                if c + stride * x > nc { continue }

                let mut tmp = grid[r][c];
                for i in range(1, stride) {
                    tmp *= grid[r + i * y][c + i * x];
                }

                if tmp > max {
                    max = tmp;
                }
            }
        }
    }

    println!("{}", max);
}

