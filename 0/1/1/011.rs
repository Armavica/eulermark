// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;
use std::path::posix::Path;

static stride: uint = 4;


fn main() {
  let path = Path::new("011.in");
  let mut file = File::open(&path);
  let contents = file.read_to_str();

  let grid =
    contents.
    lines().
    map(|line| line.words().filter_map(from_str::<int>).to_owned_vec()).
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

