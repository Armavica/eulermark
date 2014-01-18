// Copyright (C) 2014 Jorge Aparicio

use std::iter::range_inclusive;
use std::num::Bounded;

#[deriving(Eq,FromPrimitive)]
enum Day {
  Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
  fn advance(&self, days: int) -> Day {
    FromPrimitive::from_int((*self as int + days) % 7).unwrap()
  }
}

#[deriving(FromPrimitive)]
enum Month {
  January, February, March, April, May, June, July, August, September, October,
  November, December
}

impl Month {
  fn days(&self, year: int) -> int {
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

impl Bounded for Month {
  fn min_value() -> Month {
    January
  }

  fn max_value() -> Month {
    December
  }
}

fn main() {
  let mut count = 0;
  let mut today = Monday;

  for year in range_inclusive(1900, 2000) {
    for m in range_inclusive(January as int, December as int) {
      let month: Month = FromPrimitive::from_int(m).unwrap();

      today = today.advance(month.days(year));

      if year != 1900 && today == Sunday {
        count += 1;
      }
    }
  }

  println!("{}", count);
}
