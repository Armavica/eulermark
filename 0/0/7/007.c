// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

static int size = 100000;
static int target = 10000;

int main() {
  int i, count = 1;
  long j, p;
  bool *sieve = (bool *)calloc(size, sizeof(bool));

  for (i = 0; i < size; i++) {
    if (!sieve[i]) {
      p = 2*i + 3;

      if (count == target) {
        printf("%d\n", p);
        break;
      }

      for (j = p*p; j < 2*size + 3; j += 2*p)
        sieve[(j - 3) / 2] = true;

      count++;
    }
  }

  free(sieve);

  return 0;
}

