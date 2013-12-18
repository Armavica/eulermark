// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>

#define ARRAY_SIZE 1000000
#define SIEVE_SIZE 2*ARRAY_SIZE + 1

int main() {
  long i, j, n = 2, p;
  char *sieve;

  sieve = (char *)calloc(ARRAY_SIZE, sizeof(char));

  for (i = 1; i < ARRAY_SIZE; i++) {
    if (sieve[i] == 0) {
      p = 2*i + 1;
      n += p;

      for (j = p*p; j < SIEVE_SIZE; j += 2*p)
        sieve[(j - 1) / 2] = 1;
    }
  }

  free(sieve);

  printf("%llu\n", n);

  return 0;
}

