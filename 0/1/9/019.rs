// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_inclusive;

#[deriving(Eq,FromPrimitive)]
enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
    fn advance(&self, days: uint) -> Day {
        FromPrimitive::from_uint((*self as uint + days) % 7).unwrap()
    }
}

#[deriving(FromPrimitive)]
enum Month {
    January, February, March, April, May, June, July, August, September,
    October, November, December
}

impl Month {
    fn days(&self, year: uint) -> uint {
        match *self {
            January | March | May | July | August | October | December => 31,
            April | June | September | November => 30,
            _ => 28 + if year % 400 == 0 {
                    1
                } else if year % 100 == 0 {
                    0
                } else if year % 4 == 0 {
                    1
                } else {
                    0
                }
        }
    }
}

fn main() {
    let mut count = 0;
    let mut today = Monday;

    for y in range_inclusive(1900u, 2000) {
        for m in range_inclusive(January as uint, December as uint) {
            let month: Month = FromPrimitive::from_uint(m).unwrap();

            today = today.advance(month.days(y));

            if y != 1900 && today == Sunday {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
