// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>

#define SIZE 200000
#define TARGET 10001

int main() {
  long long i, j;
  int n = 0;
  char *sieve;

  sieve = (char *)calloc(SIZE, sizeof(char));

  for (i = 2; i < SIZE; i++) {
    if (sieve[i] == 0) {
      n++;

      if (n == TARGET)
        break;

      for (j = i*i; j < SIZE; j += i)
        sieve[j] = 1;
    }
  }

  free(sieve);

  if (n == TARGET)
    printf("%d\n", i);
  else
    printf("FAILED\n");

  return 0;
}

