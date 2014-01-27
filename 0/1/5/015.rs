// Copyright (C) 2014 Jorge Aparicio

use std::vec;

fn main() {
    let size = 21;
    let mut grid = vec::from_elem(size, vec::from_elem(size, 1u));

    for i in range(1, size) {
        for j in range(1, size) {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }

    println!("{}", *grid.last().unwrap().last().unwrap());
}

