// Copyright (C) 2014 Jorge Aparicio

fn main() {
    let size = 21;
    let mut line = Vec::from_elem(size, 1u);

    for _ in range(1, size) {
        line = line.move_iter().scan(0, |l, i| {*l = *l + i; Some(*l)}).collect();
    }

    println!("{}", line.last().unwrap());
}

