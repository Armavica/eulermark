// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>

#define SIZE 1000000

int collatzLength(long, int*);

int main() {
  int *collatz;
  int i, ans, tmp, max = 0;

  collatz = calloc(SIZE + 1, sizeof(int));
  collatz[1] = 1;

  for (i = 1; i <= SIZE; i++) {
    tmp = collatzLength(i, collatz);

    if (tmp > max) {
      max = tmp;
      ans = i;
    }
  }

  free(collatz);

  printf("%d\n", ans);

  return 0;
}

int collatzLength(long n, int* collatz) {
  long next = n % 2 ? 3*n + 1 : n / 2;

  if (n <= SIZE)
    if (collatz[n])
      return collatz[n];
    else
      return collatz[n] = 1 + collatzLength(next, collatz);
  else
      return 1 + collatzLength(next, collatz);
}

