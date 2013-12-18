// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>

#define ARRAY_SIZE 100000
#define SIEVE_SIZE 2 * ARRAY_SIZE + 1
#define TARGET 10001

int main() {
  long i, j, p;
  int n = 1;
  char *sieve;

  sieve = (char *)calloc(ARRAY_SIZE, sizeof(char));

  for (i = 1; i < ARRAY_SIZE; i++) {
    if (sieve[i] == 0) {
      p = 2*i + 1;

      n++;

      if (n == TARGET)
        break;

      for (j = p*p; j < SIEVE_SIZE; j += 2*p)
        sieve[(j - 1) / 2] = 1;
    }
  }

  free(sieve);

  if (n == TARGET)
    printf("%d\n", p);
  else
    printf("FAILED\n");

  return 0;
}

