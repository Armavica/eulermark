// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

int main() {
  long long n = 600851475143;
  int factor = 2;

  while (1) {
    if (n % factor == 0)
      n /= factor;
    else
      factor += 1;

    if (factor * factor > n) {
      factor = n;
      break;
    }

    if (n == 1)
      break;
  }

  printf("%d\n", factor);

  return 0;
}
