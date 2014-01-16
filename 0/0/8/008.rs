// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;

static stride: int = 5;

fn main() {
  let (mut max, mut pos) = (0, 0);
  let mut factors: [int, ..stride] = [0, ..stride];

  let path = &Path::new("008.in");
  let mut file = File::open(path);

  for byte in file.bytes() {
    if byte != '\n' as u8 {
      let num = byte as int - 48;

      factors[pos] = num;

      let prod = factors.iter().fold(1, |a, &b| a * b);

      if prod > max {
        max = prod;
      }

      pos = (pos + 1) % stride;
    }
  }

  println!("{}", max);
}

