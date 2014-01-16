// Copyright (C) 2013 Jorge Aparicio

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static int size = 1000000;

int main() {
  long i, j, acc = 2, p;
  bool *sieve = (bool *)calloc(size, sizeof(char));

  for (i = 0; i < size; i++) {
    if (!sieve[i]) {
      p = 2*i + 3;

      acc += p;

      for (j = p*p; j < 2*size + 3; j += 2*p)
        sieve[(j - 3) / 2] = true;
    }
  }

  free(sieve);

  printf("%llu\n", acc);

  return 0;
}

