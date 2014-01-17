// Copyright (C) 2014 Jorge Aparicio

// Factors(~[(a, x), (b, y)]) <-> a^x * b^y
struct Factors(~[(uint, int)]);

impl Factors {
  fn number_of_divisors(&self) -> int {
    let &Factors(ref s) = self;

    let mut nod = 1;
    for i in range(0, s.len()) {
      let (_, x) = s[i];
      nod *= x + 1;
    }

    nod
  }
}

impl Mul<Factors, Factors> for Factors {
  fn mul(&self, rhs: &Factors) -> Factors {
    let &Factors(ref s) = self;
    let &Factors(ref r) = rhs;

    let (m, n) = (s.len(), r.len());
    let (mut i, mut j) = (0 ,0);

    let mut o = ~[];
    while i != m || j != n {
      if i == m {
        o.push(r[j]);
        j += 1;
      } else if j == n {
        o.push(s[i]);
        i += 1;
      } else {
        let (a, x) = s[i];
        let (b, y) = r[j];

        if a > b {
          o.push((b, y));
          j += 1;
        } else if a == b {
          if x + y > 0 {
            o.push((a, x + y));
          }

          i += 1;
          j += 1;
        } else {
          o.push((a, x));
          i += 1;
        }
      }
    }

    Factors(o)
  }
}


fn main() {
  let mut primes = ~[2];
  let mut n: uint = 3;
  let mut next_factors = Factors(~[(2, 1)]);

  loop {
    let mut factors = ~[];

    // factorize
    let mut tmp = n.clone();
    for &prime in primes.iter() {
      if tmp == 1 {
        break;
      }

      if prime * prime > tmp {
        factors.push((tmp, 1));
        break;
      }

      let mut i = 0;
      while tmp % prime == 0 {
        i += 1;
        tmp /= prime;
      }

      if i != 0 {
        factors.push((prime, i));
      }
    }

    // new prime
    if tmp != 1 {
      primes.push(tmp);
    }

    // triangle number
    let curr_factors = next_factors;
    next_factors = Factors(factors);
    let triangle_factors = next_factors * curr_factors * Factors(~[(2, -1)]);

    // number of divisors
    let nod = triangle_factors.number_of_divisors();
    if nod > 500 {
      break;
    }

    n += 1;
  }

  println!("{}", n * (n - 1) / 2);
}

