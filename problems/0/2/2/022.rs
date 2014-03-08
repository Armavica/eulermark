// Copyright (C) 2014 Jorge Aparicio

use std::io::fs::File;
use std::iter::AdditiveIterator;

fn main() {
    let mut file = File::open(&Path::new("022.in")).ok().unwrap();
    let contents = file.read_to_str().ok().unwrap();
    let mut names = contents.split(',').map(|x| x.slice(1, x.len() - 1)).to_owned_vec();

    names.sort();

    println!("{}", names.iter().enumerate().fold(0, |a, (n, &s)| a + (n + 1) * name_value(s)));
}

fn name_value(s: &str) -> uint {
    let offset = 'A' as uint - 1;
    s.bytes().map(|x| x as uint - offset).sum()
}

