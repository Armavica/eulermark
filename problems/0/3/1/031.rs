// Copyright (C) 2014 Jorge Aparicio

extern crate collections;

use collections::HashSet;

static NPENCES: u8 = 8;

enum Pence {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty,
    OneHundred,
    TwoHundred,
}

static PENCES: [Pence, ..NPENCES] = [One, Two, Five, Ten, Twenty, Fifty,
    OneHundred, TwoHundred];

impl Pence {
    fn value(&self) -> u8 {
        match *self {
            One => 1,
            Two => 2,
            Five => 5,
            Ten => 10,
            Twenty => 20,
            Fifty => 50,
            OneHundred => 100,
            TwoHundred => 200,
        }
    }
}

#[deriving(Clone,TotalEq,Eq,Hash)]
struct Change {
    pences: Vec<u8>,
}

impl Change {
    fn new(amount: u8) -> Change {
        let mut pences = Vec::from_elem(NPENCES as uint, 0u8);
        *pences.get_mut(One as uint) = amount;

        Change { pences: pences }
    }

    fn expand(&self) -> Vec<Change> {
        let mut out = Vec::new();

        for &pence in PENCES.iter().skip(1) {
            let v = pence.value();

            if *self.pences.get(One as uint) >= v {
                let mut tmp = self.clone();

                *tmp.pences.get_mut(One as uint) -= v;
                *tmp.pences.get_mut(pence as uint) += 1;

                out.push(tmp);
            }
        }

        out
    }
}

fn main() {
    let mut row = HashSet::new();
    let mut count = 0;

    row.insert(Change::new(200));

    while row.len() > 0 {
        let mut tmp = HashSet::new();
        count += row.len();

        for parent in row.iter() {
            let children = parent.expand();

            for child in children.move_iter() {
                tmp.insert(child);
            }
        }

        row = tmp;
    }

    println!("{}", count);
}

