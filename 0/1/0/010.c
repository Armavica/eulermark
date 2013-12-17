// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>

#define SIZE 2000000

int main() {
  long i, j, n = 0;
  char *sieve;

  sieve = (char *)calloc(SIZE, sizeof(char));

  for (i = 2; i < SIZE; i++) {
    if (sieve[i] == 0) {
      n += i;

      for (j = i*i; j < SIZE; j += i)
        sieve[j] = 1;
    }
  }

  free(sieve);

  printf("%llu\n", n);

  return 0;
}

