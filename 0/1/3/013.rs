// Copyright (C) 2014 Jorge Aparicio

extern mod extra;

use extra::bigint::BigInt;
use std::io::fs::File;
use std::iter::AdditiveIterator;
use std::path::posix::Path;

fn main() {
  let path = Path::new("013.in");
  let mut file = File::open(&path);
  let contents = file.read_to_str();

  let sum = contents.
    lines().
    filter_map(|line| from_str::<BigInt>(line)).
    sum().
    to_str();

  println!("{}", sum.slice(0, 10));
}

