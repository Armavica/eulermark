// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

int main() {
  int acc = 0;

  for (int i = 1; i < 1000; i++) {
    if (i % 3 == 0 || i % 5 == 0) {
      acc += i;
    }
  }

  printf("%d\n", acc);

  return 0;
}
