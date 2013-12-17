// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

int main() {
  int i, sumOfSquares = 0, sumSquared = 0;

  for (i = 0; i < 101; i++) {
    sumSquared += i;
    sumOfSquares += i * i;
  }
  sumSquared *= sumSquared;

  printf("%d\n", sumSquared - sumOfSquares);

  return 0;
}

