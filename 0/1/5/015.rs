// Copyright (C) 2014 Jorge Aparicio

static size: uint = 21;

fn main() {
  let mut grid = ~[0, ..size * size];

  for i in range(0, size) {
    for j in range(0, size) {
      grid[index(i, j)] = if i == 0 || j == 0 {
        1
      } else {
        grid[index(i - 1, j)] + grid[index(i, j - 1)]
      }
    }
  }

  println!("{}", *grid.last());
}

fn index(i: uint, j: uint) -> uint {
  i * size + j
}

