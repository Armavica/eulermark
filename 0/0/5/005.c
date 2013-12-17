// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

long gcd(long, long);
long lcm(long, long);

int main() {
  long i, n = 1;

  for (i = 1; i < 21; i++)
    n = lcm(n, i);

  printf("%d\n", n);

  return 0;
}

long gcd(long a, long b) {
  long t, r;

  if (a < b) {
    t = a;
    a = b;
    b = t;
  }

  while (r = a%b) {
    a = b;
    b = r;
  }

  return b;
}

long lcm(long a, long b) {
  return a * b / gcd(a, b);
}

